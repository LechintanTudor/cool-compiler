use crate::paths::ModulePaths;
use crate::{CompileError, SourceFile};
use cool_lexer::lexer::{LineOffsets, Tokenizer};
use cool_lexer::symbols::Symbol;
use cool_parser::item::{DeclKind, Item, ModuleContent, ModuleKind};
use cool_parser::Parser;
use cool_resolve::item::{ItemError, ItemErrorKind, ItemId, ItemPathBuf, ItemTable};
use cool_resolve::ty::TyTable;
use std::collections::VecDeque;
use std::path::Path;

#[derive(Debug)]
pub struct Package {
    pub items: ItemTable,
    pub tys: TyTable,
    pub sources: Vec<SourceFile>,
}

pub fn compile(package_name: &str, path: &Path) -> Result<Package, CompileError> {
    let root_symbol = Symbol::insert(package_name);
    let root_paths = ModulePaths::for_root(path).unwrap();

    let mut items = ItemTable::with_builtins();
    let root_module_id = items.insert_root_module(root_symbol).unwrap();

    let tys = TyTable::with_builtins();

    let mut sources = Vec::<SourceFile>::new();

    let mut file_modules_to_process = VecDeque::<(ItemId, ModulePaths)>::new();
    file_modules_to_process.push_front((root_module_id, root_paths));

    // Module ID of the use declaration, whether the use is exported, use path
    let mut imports_to_resolve = VecDeque::<(ItemId, (bool, ItemPathBuf))>::new();

    while let Some((module_id, module_paths)) = file_modules_to_process.pop_front() {
        let source = parse_source_file(module_paths.clone());

        let mut modules_to_process = VecDeque::<(ItemId, &ModuleContent)>::new();
        modules_to_process.push_back((module_id, &source.module));

        while let Some((module_id, module_content)) = modules_to_process.pop_front() {
            for decl in module_content.decls.iter() {
                let is_exported = decl.is_exported;

                match &decl.kind {
                    DeclKind::Item(decl) => match &decl.item {
                        Item::Module(child_module) => {
                            let child_module_id = items
                                .insert_module(module_id, is_exported, decl.ident.symbol)
                                .unwrap();

                            match &child_module.kind {
                                ModuleKind::Inline(module_content) => {
                                    modules_to_process.push_back((child_module_id, module_content));
                                }
                                ModuleKind::External => {
                                    let child_module_paths = ModulePaths::for_child(
                                        &module_paths.child_module_dir,
                                        decl.ident.symbol.as_str(),
                                    )
                                    .unwrap();

                                    file_modules_to_process
                                        .push_back((child_module_id, child_module_paths));
                                }
                            }
                        }
                        Item::Fn(_) => {
                            items
                                .insert_item(module_id, is_exported, decl.ident.symbol)
                                .unwrap();
                        }
                    },
                    DeclKind::Use(decl) => {
                        let item_path = decl
                            .path
                            .idents
                            .iter()
                            .map(|ident| ident.symbol)
                            .collect::<ItemPathBuf>();

                        imports_to_resolve.push_back((module_id, (is_exported, item_path)));
                    }
                }
            }
        }

        sources.push(source);
    }

    let mut import_errors = Vec::<ItemError>::new();

    while !imports_to_resolve.is_empty() {
        let mut solved_any_import = false;
        let initial_import_count = imports_to_resolve.len();

        for _ in 0..initial_import_count {
            let (module_id, (is_exported, item_path)) = imports_to_resolve.pop_front().unwrap();

            match items.insert_use_decl(module_id, is_exported, item_path.as_path()) {
                Ok(true) => solved_any_import = true,
                Ok(false) => imports_to_resolve.push_back((module_id, (is_exported, item_path))),
                Err(error) => import_errors.push(error),
            }
        }

        if !solved_any_import {
            break;
        }
    }

    for (module_id, (_, import_path)) in imports_to_resolve {
        let module_path = items.get_path_by_id(module_id).unwrap();

        import_errors.push(ItemError {
            kind: ItemErrorKind::SymbolNotFound,
            module_path: module_path.to_path_buf(),
            symbol_path: import_path,
        });
    }

    if !import_errors.is_empty() {
        return Err(CompileError { import_errors });
    }

    Ok(Package {
        items,
        tys,
        sources,
    })
}

fn parse_source_file(module_paths: ModulePaths) -> SourceFile {
    let content = std::fs::read_to_string(&module_paths.module_path).unwrap();
    let mut line_offsets = LineOffsets::default();
    let mut tokenizer = Tokenizer::new(&content, &mut line_offsets);

    let token_iter =
        std::iter::repeat_with(|| tokenizer.next_token()).filter(|token| token.kind.is_lang_part());
    let mut parser = Parser::new(token_iter);
    let module = parser.parse_module_file().unwrap();

    SourceFile {
        module_path: module_paths.module_path,
        child_module_dir: module_paths.child_module_dir,
        content,
        line_offsets,
        module,
    }
}
