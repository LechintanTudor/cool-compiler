use crate::{
    CompileResult, ModulePaths, ParsedAlias, ParsedCrate, ParsedFn, ParsedLiteral, ParsedStruct,
    SourceFile,
};
use cool_collections::SmallVec;
use cool_derive::Section;
use cool_lexer::Symbol;
use cool_parser::{DeclKind, Item, Module, ModuleKind};
use cool_resolve::{tys, ConstItemValue, ModuleId, ResolveContext, TyConfig};
use cool_span::Span;
use std::collections::VecDeque;
use std::path::Path;

#[derive(Clone, Section, Debug)]
struct Import {
    pub span: Span,
    pub module_id: ModuleId,
    pub is_exported: bool,
    pub path: SmallVec<Symbol, 4>,
    pub alias: Option<Symbol>,
}

pub fn p0_parse(
    name: &str,
    path: &Path,
    ty_config: TyConfig,
) -> CompileResult<(ParsedCrate, ResolveContext<'static>)> {
    let mut context = ResolveContext::new_leak(ty_config);

    let root_id = context.add_root_module(Symbol::insert(name))?;
    let root_paths = ModulePaths::for_root(path)?;

    let mut file_module_queue = VecDeque::<(ModuleId, ModulePaths)>::new();
    file_module_queue.push_back((root_id, root_paths));

    let mut parsed_crate = ParsedCrate::default();
    let mut imports = VecDeque::<Import>::new();

    while let Some((module_id, module_paths)) = file_module_queue.pop_front() {
        let source_file = SourceFile::from_paths(module_paths)?;
        let module = cool_parser::parse_module(&source_file.source)?;
        let source_id = parsed_crate.files.push(source_file);

        let mut modules = VecDeque::<(ModuleId, Module)>::new();
        modules.push_back((module_id, module));

        while let Some((module_id, module)) = modules.pop_front() {
            for decl in module.decls {
                match decl.kind {
                    DeclKind::Item(item_decl) => {
                        match item_decl.item {
                            Item::Module(module) => {
                                let module_id = context.add_module(
                                    module_id,
                                    decl.is_exported,
                                    item_decl.ident.symbol,
                                )?;

                                match module.kind {
                                    ModuleKind::Extern => {
                                        let module_paths = ModulePaths::for_child(
                                            &parsed_crate.files[source_id].paths.child_dir,
                                            item_decl.ident.symbol.as_str(),
                                        )?;

                                        file_module_queue.push_back((module_id, module_paths));
                                    }
                                    ModuleKind::Inline(module) => {
                                        modules.push_back((module_id, module));
                                    }
                                }
                            }
                            Item::Struct(struct_item) => {
                                let item_id = context.add_struct(
                                    module_id,
                                    decl.is_exported,
                                    item_decl.ident.symbol,
                                )?;

                                parsed_crate.structs.push_back(ParsedStruct {
                                    source_id,
                                    span: struct_item.span,
                                    module_id,
                                    item_id,
                                    ty: item_decl.ty,
                                    item: struct_item,
                                });
                            }
                            Item::Fn(fn_item) => {
                                let item_id = context.add_const(
                                    module_id,
                                    decl.is_exported,
                                    item_decl.ident.symbol,
                                    tys::infer,
                                    ConstItemValue::Fn,
                                )?;

                                parsed_crate.fns.push_back(ParsedFn {
                                    source_id,
                                    span: fn_item.span,
                                    module_id,
                                    item_id,
                                    ty: item_decl.ty,
                                    item: fn_item,
                                });
                            }
                            Item::Literal(literal) => {
                                let item_id = context.add_const(
                                    module_id,
                                    decl.is_exported,
                                    item_decl.ident.symbol,
                                    tys::infer,
                                    ConstItemValue::Undefined,
                                )?;

                                parsed_crate.literals.push_back(ParsedLiteral {
                                    source_id,
                                    span: literal.span,
                                    module_id,
                                    item_id,
                                    ty: item_decl.ty,
                                    item: literal,
                                });
                            }
                            Item::Alias(alias) => {
                                let item_id = context.add_alias(
                                    module_id,
                                    decl.is_exported,
                                    item_decl.ident.symbol,
                                )?;

                                parsed_crate.aliases.push_back(ParsedAlias {
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

                        imports.push_back(Import {
                            span: use_decl.span,
                            module_id,
                            is_exported: decl.is_exported,
                            path,
                            alias: use_decl.alias.map(|ident| ident.symbol),
                        });
                    }
                }
            }
        }
    }

    let mut made_progress = true;

    'import_loop: while made_progress {
        let imports_len = imports.len();
        made_progress = false;

        for _ in 0..imports_len {
            let Some(import) = imports.pop_front() else {
                break 'import_loop;
            };

            let added_import = context
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
                imports.push_back(import);
            }
        }
    }

    if !imports.is_empty() {
        panic!("Unresolved imports: {:#?}", imports);
    }

    Ok((parsed_crate, context))
}
