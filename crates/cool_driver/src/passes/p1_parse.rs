use crate::paths::ModulePaths;
use crate::{
    Alias, CompileError, CompileErrorBundle, CompileOptions, Const, Enum, ExternFn, ImportError,
    ModuleError, Package, Struct,
};
use cool_lexer::Symbol;
use cool_parser::{DeclKind, Item, ModuleContent, ModuleKind};
use cool_resolve::{ItemPathBuf, ModuleId, Mutability, ResolveContext, ResolveErrorKind};
use cool_span::{Section, Span};
use std::collections::VecDeque;

#[derive(Clone, Debug)]
struct Import {
    pub span: Span,
    pub module_id: ModuleId,
    pub is_exported: bool,
    pub path: ItemPathBuf,
    pub alias: Option<Symbol>,
}

pub fn p1_parse(
    resolve: &mut ResolveContext,
    options: &CompileOptions,
) -> Result<Package, (Package, CompileErrorBundle)> {
    let mut errors = Vec::<CompileError>::new();
    let mut package = Package::default();

    let crate_paths = match ModulePaths::for_root(&options.crate_root_file) {
        Ok(crate_paths) => Some(crate_paths),
        Err(error) => {
            errors.push(CompileError::from(ModuleError {
                span: None,
                module_name: Symbol::insert(&options.crate_name),
                error,
            }));
            None
        }
    };

    let crate_symbol = Symbol::insert(&options.crate_name);
    let crate_module_id = match resolve.insert_root_module(crate_symbol) {
        Ok(crate_module_id) => Some(crate_module_id),
        Err(error) => {
            errors.push(error.into());
            None
        }
    };

    let (crate_paths, crate_module_id) = match (crate_paths, crate_module_id) {
        (Some(crate_paths), Some(crate_module_id)) => (crate_paths, crate_module_id),
        _ => return Err((package, CompileErrorBundle { errors })),
    };

    let mut file_modules = VecDeque::<(ModuleId, ModulePaths)>::new();
    file_modules.push_back((crate_module_id, crate_paths));

    let mut imports = VecDeque::<Import>::new();

    while let Some((module_id, module_paths)) = file_modules.pop_front() {
        let module_content = match package.source_map.add_file(module_paths.path.clone()) {
            Ok(source_file) => source_file,
            Err(error) => {
                errors.push(error.into());
                continue;
            }
        };

        let mut modules = VecDeque::<(ModuleId, ModuleContent)>::new();
        modules.push_back((module_id, module_content));

        while let Some((module_id, module)) = modules.pop_front() {
            for decl in module.decls {
                match decl.kind {
                    DeclKind::Item(item_decl) => {
                        let item_decl_span = item_decl.span();

                        match item_decl.item {
                            Item::Module(child_module) => {
                                let child_module_id = match resolve.insert_module(
                                    module_id,
                                    decl.is_exported,
                                    item_decl.ident.symbol,
                                ) {
                                    Ok(child_module_id) => child_module_id,
                                    Err(error) => {
                                        errors.push(error.into());
                                        continue;
                                    }
                                };

                                match child_module.kind {
                                    ModuleKind::Inline(child_module_content) => {
                                        modules.push_back((child_module_id, child_module_content));
                                    }
                                    ModuleKind::External => {
                                        let child_module_paths = match ModulePaths::for_child(
                                            &module_paths.child_dir,
                                            item_decl.ident.symbol.as_str(),
                                        ) {
                                            Ok(child_module_paths) => child_module_paths,
                                            Err(error) => {
                                                errors.push(CompileError::from(ModuleError {
                                                    span: Some(item_decl_span),
                                                    module_name: item_decl.ident.symbol,
                                                    error,
                                                }));
                                                continue;
                                            }
                                        };

                                        file_modules
                                            .push_back((child_module_id, child_module_paths));
                                    }
                                }
                            }
                            Item::Alias(item) => {
                                let item_id = match resolve.declare_alias(
                                    module_id,
                                    decl.is_exported,
                                    item_decl.ident.symbol,
                                ) {
                                    Ok(item_id) => item_id,
                                    Err(error) => {
                                        errors.push(error.into());
                                        continue;
                                    }
                                };

                                package.aliases.push(Alias {
                                    span: item_decl_span,
                                    module_id,
                                    item_id,
                                    ty: item_decl.ty,
                                    item,
                                });
                            }
                            Item::Struct(item) => {
                                let item_id = match resolve.declare_struct(
                                    module_id,
                                    decl.is_exported,
                                    item_decl.ident.symbol,
                                    item.has_body,
                                ) {
                                    Ok(item_id) => item_id,
                                    Err(error) => {
                                        errors.push(error.into());
                                        continue;
                                    }
                                };

                                package.structs.push(Struct {
                                    span: item_decl_span,
                                    module_id,
                                    item_id,
                                    ty: item_decl.ty,
                                    item,
                                });
                            }
                            Item::Enum(item) => {
                                let item_id = match resolve.declare_enum(
                                    module_id,
                                    decl.is_exported,
                                    item_decl.ident.symbol,
                                ) {
                                    Ok(item_id) => item_id,
                                    Err(error) => {
                                        errors.push(error.into());
                                        continue;
                                    }
                                };

                                package.enums.push(Enum {
                                    span: item_decl_span,
                                    module_id,
                                    item_id,
                                    ty: item_decl.ty,
                                    item,
                                });
                            }
                            Item::ExternFn(item) => {
                                let item_id = match resolve.insert_global_binding(
                                    module_id,
                                    decl.is_exported,
                                    Mutability::Const,
                                    item_decl.ident.symbol,
                                ) {
                                    Ok(item_id) => item_id,
                                    Err(error) => {
                                        errors.push(error.into());
                                        continue;
                                    }
                                };

                                package.extern_fns.push(ExternFn {
                                    span: item_decl_span,
                                    module_id,
                                    item_id,
                                    ty: item_decl.ty,
                                    item,
                                });
                            }
                            Item::Const(item) => {
                                let item_id = match resolve.insert_global_binding(
                                    module_id,
                                    decl.is_exported,
                                    Mutability::Const,
                                    item_decl.ident.symbol,
                                ) {
                                    Ok(item_id) => item_id,
                                    Err(error) => {
                                        errors.push(error.into());
                                        continue;
                                    }
                                };

                                package.consts.push(Const {
                                    span: item_decl_span,
                                    module_id,
                                    item_id,
                                    ty: item_decl.ty,
                                    item,
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
                            .collect::<ItemPathBuf>();

                        let alias = use_decl.alias.map(|alias| alias.symbol);

                        imports.push_back(Import {
                            span: use_decl.span(),
                            module_id,
                            is_exported: decl.is_exported,
                            path,
                            alias,
                        });
                    }
                }
            }
        }
    }

    let mut import_fail_count = 0_usize;
    while let Some(import) = imports.pop_front() {
        match resolve.insert_use(
            import.module_id,
            import.is_exported,
            import.path.as_symbol_slice(),
            import.alias,
        ) {
            Ok(_) => import_fail_count = 0,
            Err(error) => {
                import_fail_count += 1;

                if error.kind == ResolveErrorKind::SymbolNotFound {
                    imports.push_back(import);
                } else {
                    errors.push(CompileError::from(ImportError {
                        span: import.span,
                        path: import.path,
                    }));
                }

                if import_fail_count >= imports.len() {
                    break;
                }
            }
        }
    }

    for import in imports.drain(..) {
        errors.push(CompileError::from(ImportError {
            span: import.span,
            path: import.path,
        }));
    }

    if errors.is_empty() {
        Ok(package)
    } else {
        Err((package, CompileErrorBundle { errors }))
    }
}
