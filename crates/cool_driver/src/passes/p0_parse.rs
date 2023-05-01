use crate::paths::ModulePaths;
use crate::{
    Alias, CompileError, CompileErrorBundle, CompileOptions, CompileResult, Const, ExternFn,
    Package, SourceFile, Struct,
};
use cool_lexer::lexer::{LexedSourceFile, Tokenizer};
use cool_lexer::symbols::Symbol;
use cool_parser::{DeclKind, Item, ModuleContent, ModuleKind, ParseResult, Parser};
use cool_resolve::{
    ItemPathBuf, ModuleId, Mutability, ResolveContext, ResolveError, ResolveErrorKind,
};
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Import {
    pub source_path: PathBuf,
    pub module_id: ModuleId,
    pub is_exported: bool,
    pub path: ItemPathBuf,
    pub alias: Option<Symbol>,
}

pub fn p0_parse(resove: &mut ResolveContext, options: &CompileOptions) -> CompileResult<Package> {
    let mut errors = Vec::<CompileError>::new();
    let mut aliases = Vec::<Alias>::new();
    let mut structs = Vec::<Struct>::new();
    let mut extern_fns = Vec::<ExternFn>::new();
    let mut consts = Vec::<Const>::new();

    let crate_paths = match ModulePaths::for_root(&options.crate_root_file) {
        Ok(crate_paths) => Some(crate_paths),
        Err(error) => {
            errors.push(CompileError {
                path: options.crate_root_file.clone(),
                kind: error.into(),
            });
            None
        }
    };

    let crate_symbol = Symbol::insert(&options.crate_name);
    let crate_module_id = match resove.insert_root_module(crate_symbol) {
        Ok(crate_module_id) => Some(crate_module_id),
        Err(error) => {
            errors.push(CompileError {
                path: options.crate_root_file.clone(),
                kind: error.into(),
            });
            None
        }
    };

    let (crate_paths, crate_module_id) = match (crate_paths, crate_module_id) {
        (Some(crate_paths), Some(crate_module_id)) => (crate_paths, crate_module_id),
        _ => return Err(CompileErrorBundle { errors }),
    };

    let mut file_modules = VecDeque::<(ModuleId, ModulePaths)>::new();
    file_modules.push_back((crate_module_id, crate_paths));

    let mut imports = VecDeque::<Import>::new();

    while let Some((module_id, module_paths)) = file_modules.pop_front() {
        let source_file = match parse_source_file(module_id, &module_paths) {
            Ok(source_file) => source_file,
            Err(error) => {
                errors.push(CompileError {
                    path: module_paths.path,
                    kind: error.into(),
                });
                continue;
            }
        };

        let mut modules = VecDeque::<(ModuleId, ModuleContent)>::new();
        modules.push_back((module_id, source_file.module));

        while let Some((module_id, module)) = modules.pop_front() {
            for decl in module.decls {
                match decl.kind {
                    DeclKind::Item(item_decl) => match item_decl.item {
                        Item::Module(child_module) => {
                            let child_module_id = match resove.insert_module(
                                module_id,
                                decl.is_exported,
                                item_decl.ident.symbol,
                            ) {
                                Ok(child_module_id) => child_module_id,
                                Err(error) => {
                                    errors.push(CompileError {
                                        path: module_paths.path.clone(),
                                        kind: error.into(),
                                    });
                                    continue;
                                }
                            };

                            match child_module.kind {
                                ModuleKind::Inline(child_module_content) => {
                                    modules.push_back((child_module_id, child_module_content));
                                }
                                ModuleKind::External => {
                                    let child_module_paths = match ModulePaths::for_child(
                                        &source_file.paths.child_dir,
                                        item_decl.ident.symbol.as_str(),
                                    ) {
                                        Ok(child_module_paths) => child_module_paths,
                                        Err(error) => {
                                            errors.push(CompileError {
                                                path: module_paths.path.clone(),
                                                kind: error.into(),
                                            });
                                            continue;
                                        }
                                    };

                                    file_modules.push_back((child_module_id, child_module_paths));
                                }
                            }
                        }
                        Item::Alias(item) => {
                            let item_id = match resove.declare_alias(
                                module_id,
                                decl.is_exported,
                                item_decl.ident.symbol,
                            ) {
                                Ok(item_id) => item_id,
                                Err(error) => {
                                    errors.push(CompileError {
                                        path: module_paths.path.clone(),
                                        kind: error.into(),
                                    });
                                    continue;
                                }
                            };

                            aliases.push(Alias {
                                module_id,
                                item_id,
                                ty: item_decl.ty,
                                item,
                            });
                        }
                        Item::Struct(item) => {
                            let item_id = match resove.declare_struct(
                                module_id,
                                decl.is_exported,
                                item_decl.ident.symbol,
                            ) {
                                Ok(item_id) => item_id,
                                Err(error) => {
                                    errors.push(CompileError {
                                        path: module_paths.path.clone(),
                                        kind: error.into(),
                                    });
                                    continue;
                                }
                            };

                            structs.push(Struct {
                                module_id,
                                item_id,
                                ty: item_decl.ty,
                                item,
                            });
                        }
                        Item::ExternFn(item) => {
                            let item_id = match resove.insert_global_binding(
                                module_id,
                                decl.is_exported,
                                Mutability::Const,
                                item_decl.ident.symbol,
                            ) {
                                Ok(item_id) => item_id,
                                Err(error) => {
                                    errors.push(CompileError {
                                        path: module_paths.path.clone(),
                                        kind: error.into(),
                                    });
                                    continue;
                                }
                            };

                            extern_fns.push(ExternFn {
                                module_id,
                                item_id,
                                ty: item_decl.ty,
                                item,
                            });
                        }
                        Item::Const(item) => {
                            let item_id = match resove.insert_global_binding(
                                module_id,
                                decl.is_exported,
                                Mutability::Const,
                                item_decl.ident.symbol,
                            ) {
                                Ok(item_id) => item_id,
                                Err(error) => {
                                    errors.push(CompileError {
                                        path: module_paths.path.clone(),
                                        kind: error.into(),
                                    });
                                    continue;
                                }
                            };

                            consts.push(Const {
                                module_id,
                                item_id,
                                ty: item_decl.ty,
                                item,
                            });
                        }
                    },
                    DeclKind::Use(use_decl) => {
                        let path = use_decl
                            .path
                            .idents
                            .iter()
                            .map(|ident| ident.symbol)
                            .collect::<ItemPathBuf>();

                        let alias = use_decl.alias.map(|alias| alias.symbol);

                        imports.push_back(Import {
                            source_path: source_file.paths.path.clone(),
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
    while let Some(import) = imports.pop_back() {
        match resove.insert_use(
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
                    errors.push(CompileError {
                        path: import.source_path,
                        kind: error.into(),
                    });
                }

                if import_fail_count >= imports.len() {
                    break;
                }
            }
        }
    }

    for import in imports.drain(..) {
        errors.push(CompileError {
            path: import.source_path,
            kind: ResolveError::not_found(import.path.last()).into(),
        });
    }

    if errors.is_empty() {
        Ok(Package {
            aliases,
            structs,
            extern_fns,
            consts,
        })
    } else {
        Err(CompileErrorBundle { errors })
    }
}

fn parse_source_file(module_id: ModuleId, paths: &ModulePaths) -> ParseResult<SourceFile> {
    let lexed = {
        let file = File::open(&paths.path).unwrap();
        let mut buf_reader = BufReader::new(file);
        LexedSourceFile::from_reader(&mut buf_reader)
    };

    let mut tokenizer = Tokenizer::new(&lexed.source);
    let mut parser = Parser::new(&lexed, tokenizer.stream());
    let module = parser.parse_module_file()?;

    println!("{:#?}", module);

    Ok(SourceFile {
        paths: paths.clone(),
        lexed,
        module_id,
        module,
    })
}
