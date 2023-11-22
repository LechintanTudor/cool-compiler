use crate::{
    ModulePaths, ParsedAlias, ParsedCrate, ParsedExternFn, ParsedFn, ParsedLiteral, ParsedStruct,
    SourceFile, SourceId, SpannedCompileError, SpannedCompileResult, WithLocation,
};
use cool_collections::SmallVec;
use cool_derive::Section;
use cool_lexer::Symbol;
use cool_parser::{Decl, DeclKind, Item, Module, ModuleKind};
use cool_resolve::{tys, ConstItemValue, ModuleId, ResolveContext, TyConfig};
use cool_span::{Section, Span};
use std::collections::VecDeque;
use std::path::Path;

pub fn p0_parse(
    ty_config: TyConfig,
    crate_name: &str,
    crate_path: &Path,
) -> (
    ResolveContext<'static>,
    ParsedCrate,
    Vec<SpannedCompileError>,
) {
    let context = ResolveContext::new_leak(ty_config);
    let mut crate_parser = CrateParser::new(context);

    crate_parser.add_crate(crate_name, crate_path);
    crate_parser.solve_imports();
    crate_parser.into_artifacts()
}

#[derive(Clone, Section, Debug)]
struct Import {
    source_id: SourceId,
    span: Span,
    module_id: ModuleId,
    is_exported: bool,
    path: SmallVec<Symbol, 4>,
    alias: Option<Symbol>,
}

struct CrateParser {
    context: ResolveContext<'static>,
    parsed_crate: ParsedCrate,
    file_module_queue: VecDeque<(ModuleId, ModulePaths)>,
    module_queue: VecDeque<(ModuleId, Module)>,
    import_queue: VecDeque<Import>,
    errors: Vec<SpannedCompileError>,
}

impl CrateParser {
    fn new(context: ResolveContext<'static>) -> Self {
        Self {
            context,
            parsed_crate: Default::default(),
            file_module_queue: Default::default(),
            module_queue: Default::default(),
            import_queue: Default::default(),
            errors: Default::default(),
        }
    }

    fn add_crate(&mut self, crate_name: &str, crate_path: &Path) {
        if let Err(error) = self.parse_crate(crate_name, crate_path) {
            self.errors.push(error);
        }
    }

    fn parse_crate(&mut self, crate_name: &str, crate_path: &Path) -> SpannedCompileResult {
        let root_paths = ModulePaths::for_root(crate_path)
            .map_err(|error| error.with_location(crate_path.to_path_buf()))?;

        let root_id = self
            .context
            .add_root_module(Symbol::insert(crate_name))
            .map_err(|error| error.with_location(crate_path.to_path_buf()))?;

        self.file_module_queue.clear();
        self.file_module_queue.push_back((root_id, root_paths));

        while let Some((module_id, module_paths)) = self.file_module_queue.pop_front() {
            if let Err(error) = self.parse_file_module(module_id, module_paths) {
                self.errors.push(error);
            }
        }

        Ok(())
    }

    fn parse_file_module(
        &mut self,
        module_id: ModuleId,
        module_paths: ModulePaths,
    ) -> SpannedCompileResult {
        let source_file = SourceFile::from_paths(module_paths.clone())
            .map_err(|error| error.with_location(module_paths.path))?;

        let source_id = self.parsed_crate.files.push(source_file);
        let source = self.parsed_crate.files.last().unwrap().source.as_str();

        let module = cool_parser::parse_module(source)
            .map_err(|error| error.with_location((source_id, error.span())))?;

        self.module_queue.clear();
        self.module_queue.push_front((module_id, module));

        while let Some((module_id, module)) = self.module_queue.pop_front() {
            for decl in module.decls {
                if let Err(error) = self.parse_decl(source_id, module_id, decl) {
                    self.errors.push(error);
                }
            }
        }

        Ok(())
    }

    fn parse_decl(
        &mut self,
        source_id: SourceId,
        module_id: ModuleId,
        decl: Decl,
    ) -> SpannedCompileResult {
        match decl.kind {
            DeclKind::Item(item_decl) => {
                match item_decl.item {
                    Item::Module(module) => {
                        let module_id = self
                            .context
                            .add_module(module_id, decl.is_exported, item_decl.ident.symbol)
                            .map_err(|error| {
                                error.with_location((source_id, item_decl.ident.span))
                            })?;

                        match module.kind {
                            ModuleKind::Extern => {
                                let module_paths = ModulePaths::for_child(
                                    &self.parsed_crate.files[source_id].paths.child_dir,
                                    item_decl.ident.symbol.as_str(),
                                )
                                .map_err(|error| {
                                    error.with_location((source_id, item_decl.ident.span))
                                })?;

                                self.file_module_queue.push_back((module_id, module_paths));
                            }
                            ModuleKind::Inline(module) => {
                                self.module_queue.push_back((module_id, module));
                            }
                        }
                    }
                    Item::Struct(struct_item) => {
                        let item_id = self
                            .context
                            .add_struct(module_id, decl.is_exported, item_decl.ident.symbol)
                            .map_err(|error| {
                                error.with_location((source_id, item_decl.ident.span))
                            })?;

                        self.parsed_crate.structs.push(ParsedStruct {
                            source_id,
                            span: struct_item.span,
                            module_id,
                            item_id,
                            ty: item_decl.ty,
                            item: struct_item,
                        });
                    }
                    Item::ExternFn(fn_item) => {
                        let item_id = self
                            .context
                            .add_const(
                                module_id,
                                decl.is_exported,
                                item_decl.ident.symbol,
                                tys::infer,
                                ConstItemValue::Fn,
                            )
                            .map_err(|error| {
                                error.with_location((source_id, item_decl.ident.span))
                            })?;

                        self.parsed_crate.extern_fns.push(ParsedExternFn {
                            source_id,
                            span: fn_item.span(),
                            module_id,
                            item_id,
                            ty: item_decl.ty,
                            item: fn_item,
                        });
                    }
                    Item::Fn(fn_item) => {
                        let item_id = self
                            .context
                            .add_const(
                                module_id,
                                decl.is_exported,
                                item_decl.ident.symbol,
                                tys::infer,
                                ConstItemValue::Fn,
                            )
                            .map_err(|error| {
                                error.with_location((source_id, item_decl.ident.span))
                            })?;

                        self.parsed_crate.fns.push(ParsedFn {
                            source_id,
                            span: fn_item.span(),
                            module_id,
                            item_id,
                            ty: item_decl.ty,
                            item: fn_item,
                        });
                    }
                    Item::Literal(literal) => {
                        let item_id = self
                            .context
                            .add_const(
                                module_id,
                                decl.is_exported,
                                item_decl.ident.symbol,
                                tys::infer,
                                ConstItemValue::Undefined,
                            )
                            .map_err(|error| {
                                error.with_location((source_id, item_decl.ident.span))
                            })?;

                        self.parsed_crate.literals.push(ParsedLiteral {
                            source_id,
                            span: literal.span,
                            module_id,
                            item_id,
                            ty: item_decl.ty,
                            item: literal,
                        });
                    }
                    Item::Alias(alias) => {
                        let item_id = self
                            .context
                            .add_alias(module_id, decl.is_exported, item_decl.ident.symbol)
                            .map_err(|error| {
                                error.with_location((source_id, item_decl.ident.span))
                            })?;

                        self.parsed_crate.aliases.push(ParsedAlias {
                            source_id,
                            span: alias.span,
                            module_id,
                            item_id,
                            ty: item_decl.ty,
                            item: alias,
                        });
                    }
                }
            }
            DeclKind::Use(use_decl) => {
                let path = use_decl
                    .path
                    .idents
                    .iter()
                    .map(|ident| ident.symbol)
                    .collect();

                self.import_queue.push_back(Import {
                    source_id,
                    span: use_decl.span,
                    module_id,
                    is_exported: decl.is_exported,
                    path,
                    alias: use_decl.alias.map(|ident| ident.symbol),
                });
            }
        }

        Ok(())
    }

    fn solve_imports(&mut self) {
        loop {
            let imports_len = self.import_queue.len();
            let mut made_progress = false;

            for _ in 0..imports_len {
                let import = self.import_queue.pop_front().unwrap();

                let added_import = self
                    .context
                    .add_import(
                        import.module_id,
                        import.is_exported,
                        &import.path,
                        import.alias,
                    )
                    .is_ok();

                if added_import {
                    made_progress = true;
                } else {
                    self.import_queue.push_back(import);
                }
            }

            if !made_progress {
                break;
            }
        }

        while let Some(import) = self.import_queue.pop_front() {
            if let Err(error) = self.context.add_import(
                import.module_id,
                import.is_exported,
                &import.path,
                import.alias,
            ) {
                self.errors
                    .push(error.with_location((import.source_id, import.span)));
            }
        }
    }

    #[inline]
    fn into_artifacts(
        self,
    ) -> (
        ResolveContext<'static>,
        ParsedCrate,
        Vec<SpannedCompileError>,
    ) {
        (self.context, self.parsed_crate, self.errors)
    }
}
