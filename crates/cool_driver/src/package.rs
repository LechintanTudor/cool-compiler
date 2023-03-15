use crate::paths::ModulePaths;
use crate::{CompileError, SourceFile};
use cool_ast::{AstGenerator, ModuleAst};
use cool_lexer::lexer::{LineOffsets, Tokenizer};
use cool_lexer::symbols::Symbol;
use cool_parser::{DeclKind, Item, ModuleContent, ModuleKind, Parser};
use cool_resolve::item::{ItemError, ItemId, ItemPathBuf, ItemTable};
use cool_resolve::ty::TyTable;
use std::collections::VecDeque;
use std::path::Path;

#[derive(Debug)]
pub struct Package {
    pub items: ItemTable,
    pub tys: TyTable,
    pub sources: Vec<SourceFile>,
}

#[derive(Clone, Debug)]
pub struct FileModuleToProcess {
    pub module_id: ItemId,
    pub paths: ModulePaths,
}

#[derive(Clone, Debug)]
pub struct ImportToResolve {
    pub module_id: ItemId,
    pub is_exported: bool,
    pub import_path: ItemPathBuf,
}

pub struct Driver<'a> {
    pub items: &'a mut ItemTable,
    pub source_files: Vec<SourceFile>,
    pub file_modules_to_process: VecDeque<FileModuleToProcess>,
    pub imports_to_resolve: VecDeque<ImportToResolve>,
}

impl<'a> Driver<'a> {
    pub fn new(items: &'a mut ItemTable, crate_name: &str, crate_path: &Path) -> Self {
        let crate_symbol = Symbol::insert(crate_name);
        let crate_paths = ModulePaths::for_root(crate_path).unwrap();
        let crate_id = items.insert_root_module(crate_symbol).unwrap();

        Self {
            items,
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

        let source_file = parse_source_file(file_module.paths, file_module.module_id);

        let mut modules_to_process = VecDeque::<(ItemId, &ModuleContent)>::new();
        modules_to_process.push_back((file_module.module_id, &source_file.module));

        while let Some((module_id, module)) = modules_to_process.pop_front() {
            for decl in module.decls.iter() {
                let is_exported = decl.is_exported;

                match &decl.kind {
                    DeclKind::Item(decl) => match &decl.item {
                        Item::Module(child_module) => {
                            let child_id = self
                                .items
                                .insert_module(module_id, is_exported, decl.ident.symbol)
                                .unwrap();

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
                            self.items
                                .insert_item(module_id, is_exported, decl.ident.symbol)
                                .unwrap();
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
        let mut import_errors = Vec::<ItemError>::new();

        while !self.imports_to_resolve.is_empty() {
            let mut solved_any_import = false;
            let initial_import_count = self.imports_to_resolve.len();

            for _ in 0..initial_import_count {
                let import = self.imports_to_resolve.pop_front().unwrap();

                match self.items.insert_use_decl(
                    import.module_id,
                    import.is_exported,
                    import.import_path.as_path(),
                ) {
                    Ok(true) => solved_any_import = true,
                    Ok(false) => self.imports_to_resolve.push_back(import),
                    Err(error) => import_errors.push(error),
                }
            }

            if !solved_any_import {
                break;
            }
        }
    }

    #[inline]
    pub fn into_source_files(self) -> Vec<SourceFile> {
        self.source_files
    }
}

fn parse_source_file(paths: ModulePaths, module_id: ItemId) -> SourceFile {
    let source = std::fs::read_to_string(&paths.path).unwrap();
    let mut line_offsets = LineOffsets::default();
    let mut tokenizer = Tokenizer::new(&source, &mut line_offsets);

    let mut parser = Parser::new(tokenizer.iter_lang_tokens());
    let module = parser.parse_module_file().unwrap();

    SourceFile {
        paths,
        line_offsets,
        source,
        module_id,
        module,
    }
}

pub fn generate_ast(package: &mut Package) -> Result<Vec<ModuleAst>, CompileError> {
    let mut ast_generator = AstGenerator {
        items: &package.items,
        tys: &mut package.tys,
    };

    let mut module_asts = Vec::<ModuleAst>::new();

    for module in package.sources.iter().map(|source| &source.module) {
        module_asts.push(ast_generator.generate_module(module));
    }

    Ok(module_asts)
}
