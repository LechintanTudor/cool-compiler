use crate::paths::ModulePaths;
use crate::SourceFile;
use cool_ast::{AstGenerator, ModuleItemAst, ResolveAst};
use cool_lexer::lexer::{LexedSourceFile, Tokenizer};
use cool_lexer::symbols::Symbol;
use cool_parser::{DeclKind, Item, ModuleContent, ModuleKind, ParseResult, Parser};
use cool_resolve::{
    tys, ItemPathBuf, ModuleId, Mutability, ResolveError, ResolveErrorKind, ResolveTable,
};
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug)]
pub struct Package {
    pub resolve: ResolveTable,
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
        let (_, crate_module_id) = resolve.insert_root_module(crate_symbol).unwrap();

        Self {
            resolve,
            source_files: Default::default(),
            file_modules_to_process: vec![FileModuleToProcess {
                module_id: crate_module_id,
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

        let mut source_file = match parse_source_file(file_module.paths, file_module.module_id) {
            Ok(source_file) => source_file,
            Err(error) => {
                println!("\n{}", error);
                return true;
            }
        };

        let mut modules_to_process = VecDeque::<(ModuleId, &mut ModuleContent)>::new();
        modules_to_process.push_back((file_module.module_id, &mut source_file.module));

        while let Some((module_id, module)) = modules_to_process.pop_front() {
            for decl in module.decls.iter_mut() {
                let is_exported = decl.is_exported;

                match &mut decl.kind {
                    DeclKind::Item(decl) => match &mut decl.item {
                        Item::Module(child_module) => {
                            let (_, child_module_id) = self
                                .resolve
                                .insert_module(module_id, is_exported, decl.ident.symbol)
                                .unwrap();

                            match &mut child_module.kind {
                                ModuleKind::Inline(module_content) => {
                                    modules_to_process.push_back((child_module_id, module_content));
                                }
                                ModuleKind::External => {
                                    todo!()
                                    // let child_paths = ModulePaths::for_child(
                                    //     &source_file.paths.child_dir,
                                    //     decl.ident.symbol.as_str(),
                                    // )
                                    // .unwrap();

                                    // self.file_modules_to_process.push_back(FileModuleToProcess {
                                    //     module_id: child_id,
                                    //     paths: child_paths,
                                    // });
                                }
                            }
                        }
                        Item::Const(_) | Item::ExternFn(_) => {
                            let (item_id, _) = self
                                .resolve
                                .insert_global_binding(
                                    module_id,
                                    is_exported,
                                    Mutability::Const,
                                    decl.ident.symbol,
                                )
                                .unwrap();

                            decl.item_id = item_id;
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

    pub fn resolve_imports(&mut self) -> Result<(), Vec<ResolveError>> {
        let mut import_errors = Vec::<ResolveError>::new();

        while !self.imports_to_resolve.is_empty() {
            let mut solved_any_import = false;
            let initial_import_count = self.imports_to_resolve.len();

            for _ in 0..initial_import_count {
                let import = self.imports_to_resolve.pop_front().unwrap();

                let resolve_result = self.resolve.insert_use(
                    import.module_id,
                    import.is_exported,
                    import.import_path.as_path(),
                    None,
                );

                match resolve_result {
                    Ok(_) => solved_any_import = true,
                    Err(error) => {
                        if error.kind == ResolveErrorKind::SymbolNotFound {
                            self.imports_to_resolve.push_back(import);
                        } else {
                            import_errors.push(error);
                        }
                    }
                }
            }

            if !solved_any_import {
                break;
            }
        }

        for import in self.imports_to_resolve.drain(..) {
            import_errors.push(ResolveError::not_found(import.import_path.last()));
        }

        if !import_errors.is_empty() {
            return Err(import_errors);
        }

        Ok(())
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
    let mut ast_generator = AstGenerator::new(&mut package.resolve);
    let mut module_asts = Vec::<ModuleItemAst>::new();

    for source in package.sources.iter() {
        module_asts.push(ast_generator.gen_module(source.module_id, &source.module));
    }

    for module_ast in module_asts.iter() {
        module_ast
            .resolve_consts(&mut ast_generator, tys::MODULE)
            .unwrap();
    }

    for module_ast in module_asts.iter() {
        module_ast
            .resolve_exprs(&mut ast_generator, tys::MODULE)
            .unwrap();
    }

    Ok(module_asts)
}
