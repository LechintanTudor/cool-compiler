use crate::paths::ModulePaths;
use crate::SourceFile;
use cool_ast::{AstGenerator, ModuleItemAst};
use cool_lexer::lexer::{LexedSourceFile, Tokenizer};
use cool_lexer::symbols::Symbol;
use cool_parser::{DeclKind, Item, ModuleContent, ModuleKind, ParseResult, Parser};
use cool_resolve::resolve::{ModuleId, ResolveTable};
use cool_resolve::ty::TyTable;
use cool_resolve::ItemPathBuf;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug)]
pub struct Package {
    pub resolve: ResolveTable,
    pub tys: TyTable,
    pub sources: Vec<SourceFile>,
}

#[derive(Clone, Debug)]
pub struct FileModuleToProcess {
    pub module_id: ModuleId,
    pub paths: ModulePaths,
}

#[derive(Clone, Debug)]
pub struct ImportToResolve {
    pub module_id: ModuleId,
    pub is_exported: bool,
    pub import_path: ItemPathBuf,
}

pub struct Driver<'a> {
    pub resolve: &'a mut ResolveTable,
    pub source_files: Vec<SourceFile>,
    pub file_modules_to_process: VecDeque<FileModuleToProcess>,
    pub imports_to_resolve: VecDeque<ImportToResolve>,
}

impl<'a> Driver<'a> {
    pub fn new(resolve: &'a mut ResolveTable, crate_name: &str, crate_path: &Path) -> Self {
        let crate_symbol = Symbol::insert(crate_name);
        let crate_paths = ModulePaths::for_root(crate_path).unwrap();
        let crate_id = resolve.add_root_module(crate_symbol);

        Self {
            resolve,
            source_files: Default::default(),
            file_modules_to_process: vec![FileModuleToProcess {
                module_id: crate_id,
                paths: crate_paths,
            }]
            .into(),
            imports_to_resolve: Default::default(),
        }
    }

    pub fn process_next_file_module(&mut self) -> bool {
        let Some(file_module) = self.file_modules_to_process.pop_front() else {
            return false;
        };

        let source_file = match parse_source_file(file_module.paths, file_module.module_id) {
            Ok(source_file) => source_file,
            Err(error) => {
                println!("\n{}", error);
                return true;
            }
        };

        let mut modules_to_process = VecDeque::<(ModuleId, &ModuleContent)>::new();
        modules_to_process.push_back((file_module.module_id, &source_file.module));

        while let Some((module_id, module)) = modules_to_process.pop_front() {
            for decl in module.decls.iter() {
                let is_exported = decl.is_exported;

                match &decl.kind {
                    DeclKind::Item(decl) => match &decl.item {
                        Item::Module(child_module) => {
                            let child_id = self.resolve.add_module_to_module(
                                module_id,
                                is_exported,
                                decl.ident.symbol,
                            );

                            match &child_module.kind {
                                ModuleKind::Inline(module_content) => {
                                    modules_to_process.push_back((child_id, module_content));
                                }
                                ModuleKind::External => {
                                    let child_paths = ModulePaths::for_child(
                                        &source_file.paths.child_dir,
                                        decl.ident.symbol.as_str(),
                                    )
                                    .unwrap();

                                    self.file_modules_to_process.push_back(FileModuleToProcess {
                                        module_id: child_id,
                                        paths: child_paths,
                                    });
                                }
                            }
                        }
                        Item::Fn(_) => {
                            self.resolve.add_item_to_module(
                                module_id,
                                is_exported,
                                decl.ident.symbol,
                            );
                        }
                    },
                    DeclKind::Use(decl) => {
                        let import_path = decl
                            .path
                            .idents
                            .iter()
                            .map(|ident| ident.symbol)
                            .collect::<ItemPathBuf>();

                        self.imports_to_resolve.push_back(ImportToResolve {
                            module_id,
                            is_exported,
                            import_path,
                        })
                    }
                }
            }
        }

        self.source_files.push(source_file);
        true
    }

    pub fn resolve_imports(&mut self) {
        // let mut import_errors = Vec::<ItemError>::new();

        // while !self.imports_to_resolve.is_empty() {
        //     let mut solved_any_import = false;
        //     let initial_import_count = self.imports_to_resolve.len();

        //     for _ in 0..initial_import_count {
        //         let import = self.imports_to_resolve.pop_front().unwrap();

        //         match self.symbols.insert_use_decl(
        //             import.module_id,
        //             import.is_exported,
        //             import.import_path.as_path(),
        //         ) {
        //             Ok(true) => solved_any_import = true,
        //             Ok(false) => self.imports_to_resolve.push_back(import),
        //             Err(error) => import_errors.push(error),
        //         }
        //     }

        //     if !solved_any_import {
        //         break;
        //     }
        // }
    }

    #[inline]
    pub fn into_source_files(self) -> Vec<SourceFile> {
        self.source_files
    }
}

fn parse_source_file(paths: ModulePaths, module_id: ModuleId) -> ParseResult<SourceFile> {
    let lexed = {
        let file = File::open(&paths.path).unwrap();
        let mut buf_reader = BufReader::new(file);
        LexedSourceFile::from_reader(&mut buf_reader)
    };

    let mut tokenizer = Tokenizer::new(&lexed.source);
    let mut parser = Parser::new(&lexed, tokenizer.iter_lang_tokens());
    let module = parser.parse_module_file()?;

    Ok(SourceFile {
        paths,
        lexed,
        module_id,
        module,
    })
}

pub fn generate_ast(package: &mut Package) -> Result<Vec<ModuleItemAst>, ()> {
    let mut ast_generator = AstGenerator {
        resolve: &mut package.resolve,
        tys: &mut package.tys,
    };

    let mut module_asts = Vec::<ModuleItemAst>::new();

    for source in package.sources.iter() {
        module_asts.push(ast_generator.generate_module(source.module_id, &source.module));
    }

    Ok(module_asts)
}
