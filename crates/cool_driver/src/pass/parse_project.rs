use crate::pass::ProjectData;
use crate::{LineOffsets, ModulePaths};
use cool_collections::{SmallVec, VecMap};
use cool_lexer::Symbol;
use cool_resolve::{CrateId, ItemId, ModuleId, ResolveContext};
use std::collections::VecDeque;
use std::io::Read;
use std::path::Path;
use std::{fs, io};

#[derive(Clone, Default, Debug)]
pub struct Project {
    pub crates: VecMap<ast::CrateId, Crate>,
    pub files: VecMap<ast::FileId, File>,
    pub items: Vec<Item>,
    pub imports: Vec<Import>,
}

#[derive(Clone, Debug)]
pub struct Crate {
    pub crate_id: CrateId,
    pub files: SmallVec<ast::FileId, 4>,
}

impl Crate {
    #[inline]
    #[must_use]
    pub fn new(crate_id: CrateId) -> Self {
        Self {
            crate_id,
            files: SmallVec::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct File {
    pub ast_crate_id: ast::CrateId,
    pub line_offsets: LineOffsets,
    pub parsed_file: ast::File,
    pub module_id: ModuleId,
}

#[derive(Clone, Debug)]
pub struct Item {
    pub ast_file_id: ast::FileId,
    pub ast_item_id: ast::ItemId,
    pub item_id: ItemId,
}

#[derive(Clone, Debug)]
pub struct Import {
    pub ast_file_id: ast::FileId,
    pub ast_import_id: ast::ImportId,
    pub module_id: ModuleId,
    pub is_exported: bool,
    pub path: SmallVec<Symbol, 4>,
    pub symbol: Symbol,
}

pub fn parse_project(data: &ProjectData, context: &mut ResolveContext) -> Project {
    let mut project = Project::default();

    let crate_ids = data
        .crates
        .iter()
        .map(|project_crate| Symbol::insert(&project_crate.name))
        .map(|crate_name| context.add_crate(crate_name))
        .collect::<SmallVec<_, 4>>();

    for (ast_crate_id, &crate_id) in data.crates.iter_indexes().zip(crate_ids.iter()) {
        for dep in &data.crates[ast_crate_id].deps {
            let dep_id = crate_ids[dep.crate_id.get() as usize];
            let dep_name = Symbol::insert(&dep.mount_name);
            context.add_dep(crate_id, dep_name, dep_id).unwrap();
        }
    }

    for (crate_id, project_crate) in crate_ids.into_iter().zip(data.crates.iter()) {
        let paths = ModulePaths::for_root(&project_crate.entry_path()).unwrap();
        let ast_crate_id = project.crates.push(Crate::new(crate_id));

        let mut file_module_queue = VecDeque::<(ModulePaths, ModuleId)>::new();
        file_module_queue.push_back((paths, crate_id.as_module_id()));

        while let Some((paths, module_id)) = file_module_queue.pop_front() {
            let (source, line_offsets) = read_file(&paths.path);
            let parsed_file = ast::parse_file(&source).unwrap();

            let ast_file_id = project.files.push(File {
                ast_crate_id,
                line_offsets,
                parsed_file,
                module_id,
            });
            project.crates[ast_crate_id].files.push(ast_file_id);

            let parsed_file = &project.files[ast_file_id].parsed_file;

            let mut module_queue = VecDeque::<(ast::ModuleId, ModuleId)>::new();
            module_queue.push_back((ast::ModuleId::new(0), module_id));

            while let Some((ast_module_id, module_id)) = module_queue.pop_front() {
                let module = &parsed_file.modules[ast_module_id];

                for &ast_decl_id in &module.decls {
                    let decl = &parsed_file.decls[ast_decl_id];

                    match parsed_file.decls[ast_decl_id].kind {
                        ast::DeclKind::Import(ast_import_id) => {
                            let (path, symbol) =
                                get_import_path_and_symbol(&parsed_file.imports[ast_import_id]);

                            project.imports.push(Import {
                                ast_file_id,
                                ast_import_id,
                                module_id,
                                is_exported: decl.is_exported,
                                path,
                                symbol,
                            });
                        }
                        ast::DeclKind::Item(ast_item_id) => {
                            let item = &parsed_file.items[ast_item_id];

                            match item.kind {
                                ast::ItemKind::Alias(_) => {
                                    let item_id = context
                                        .add_alias(module_id, decl.is_exported, item.ident.symbol)
                                        .unwrap();

                                    project.items.push(Item {
                                        ast_file_id,
                                        ast_item_id,
                                        item_id,
                                    });
                                }
                                ast::ItemKind::Expr(_) | ast::ItemKind::ExternFn(_) => {
                                    let item_id = context
                                        .add_global_binding(
                                            module_id,
                                            decl.is_exported,
                                            item.ident.symbol,
                                        )
                                        .unwrap();

                                    project.items.push(Item {
                                        ast_file_id,
                                        ast_item_id,
                                        item_id,
                                    });
                                }
                                ast::ItemKind::Module(ast_module_id) => {
                                    let item_id = context
                                        .add_module(module_id, decl.is_exported, item.ident.symbol)
                                        .unwrap();

                                    let module_id = context[item_id].into_module();
                                    module_queue.push_back((ast_module_id, module_id));
                                }
                                ast::ItemKind::Struct(_) => {
                                    let item_id = context
                                        .add_struct(module_id, decl.is_exported, item.ident.symbol)
                                        .unwrap();

                                    project.items.push(Item {
                                        ast_file_id,
                                        ast_item_id,
                                        item_id,
                                    });
                                }
                                ast::ItemKind::ExternModule => {
                                    let item_id = context
                                        .add_module(module_id, decl.is_exported, item.ident.symbol)
                                        .unwrap();

                                    let module_id = context[item_id].into_module();

                                    let module_paths = ModulePaths::for_child(
                                        &paths.child_dir,
                                        item.ident.symbol.as_str(),
                                    )
                                    .unwrap();

                                    file_module_queue.push_back((module_paths, module_id));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    project
}

fn read_file(path: &Path) -> (String, LineOffsets) {
    let mut reader = fs::File::open(path)
        .map(io::BufReader::new)
        .expect("Failed to open file");

    let mut source = String::new();
    let mut line_offsets = LineOffsets::default();

    loop {
        match reader.read_to_string(&mut source) {
            Ok(0) => break,
            Ok(n) => line_offsets.add_line(n as u32),
            Err(error) => panic!("Failed to read file: {error}"),
        }
    }

    (source, line_offsets)
}

fn get_import_path_and_symbol(import: &ast::Import) -> (SmallVec<Symbol, 4>, Symbol) {
    let path = import
        .path
        .idents
        .iter()
        .map(|ident| ident.symbol)
        .collect::<SmallVec<_, 4>>();

    let symbol = import
        .alias
        .map_or_else(|| *path.last().unwrap(), |alias| alias.symbol);

    (path, symbol)
}
