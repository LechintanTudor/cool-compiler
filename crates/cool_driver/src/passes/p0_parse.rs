use crate::paths::ModulePaths;
use crate::{CompileError, CompileErrorBundle, CompileOptions, CompileResult, SourceFile};
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

#[derive(Debug)]
pub struct Package {
    pub root_file: PathBuf,
    pub sources: Vec<SourceFile>,
}

#[derive(Clone, Debug)]
pub struct Import {
    pub source_path: PathBuf,
    pub module_id: ModuleId,
    pub is_exported: bool,
    pub path: ItemPathBuf,
    pub alias: Option<Symbol>,
}

pub fn p0_parse(resove: &mut ResolveContext, options: &CompileOptions) -> CompileResult<Package> {
    let mut package = Package {
        root_file: options.crate_root_file.clone(),
        sources: Default::default(),
    };

    let mut errors = Vec::<CompileError>::new();

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
        _ => return Err(CompileErrorBundle { package, errors }),
    };

    let mut file_modules = VecDeque::<(ModuleId, ModulePaths)>::new();
    file_modules.push_back((crate_module_id, crate_paths));

    let mut imports = VecDeque::<Import>::new();

    while let Some((module_id, module_paths)) = file_modules.pop_front() {
        let mut source_file = match parse_source_file(module_id, &module_paths) {
            Ok(source_file) => source_file,
            Err(error) => {
                errors.push(CompileError {
                    path: module_paths.path,
                    kind: error.into(),
                });
                continue;
            }
        };

        let mut modules = VecDeque::<(ModuleId, &mut ModuleContent)>::new();
        modules.push_back((module_id, &mut source_file.module));

        while let Some((module_id, module)) = modules.pop_front() {
            for decl in module.decls.iter_mut() {
                match &mut decl.kind {
                    DeclKind::Item(item_decl) => match &mut item_decl.item {
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

                            match &mut child_module.kind {
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
                        Item::Alias(_) => {
                            item_decl.item_id = match resove.declare_alias(
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
                        }
                        Item::Struct(_) => {
                            item_decl.item_id = match resove.declare_struct(
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
                        }
                        Item::Const(_) => {
                            item_decl.item_id = match resove.insert_global_binding(
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
                            }
                        }
                        _ => todo!(),
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

        package.sources.push(source_file);
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
        Ok(package)
    } else {
        Err(CompileErrorBundle { package, errors })
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

    Ok(SourceFile {
        paths: paths.clone(),
        lexed,
        module_id,
        module,
    })
}
