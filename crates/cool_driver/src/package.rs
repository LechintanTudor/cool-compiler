use crate::paths::ModulePaths;
use crate::SourceFile;
use cool_lexer::lexer::{LineOffsets, Tokenizer};
use cool_lexer::symbols::Symbol;
use cool_parser::item::{DeclKind, Item, ModuleContent, ModuleKind};
use cool_parser::Parser;
use cool_resolve::item::{ItemId, ItemPathBuf, ItemTable};
use std::collections::VecDeque;
use std::path::Path;

// TODO:
// Then solve imports and use's
// The generate AST

#[derive(Debug)]
pub struct Package {
    pub items: ItemTable,
    pub sources: Vec<SourceFile>,
}

pub fn compile(package_name: &str, path: &Path) -> Package {
    let mut items = ItemTable::default();
    let mut sources = Vec::<SourceFile>::new();

    let package_symbol = Symbol::insert(package_name);
    let package_paths = ModulePaths::for_root(path).unwrap();

    let mut files_to_process = VecDeque::<(ItemPathBuf, ModulePaths)>::new();
    files_to_process.push_front((ItemPathBuf::from(package_symbol), package_paths));

    // Module ID of the use declaration, whether the use is exported, use path
    let mut uses_to_resolve = VecDeque::<(ItemId, (bool, ItemPathBuf))>::new();

    while let Some((item_path, module_paths)) = files_to_process.pop_front() {
        let source = parse_source_file(item_path.clone(), module_paths.clone());

        let mut modules_to_process = VecDeque::<(ItemPathBuf, &ModuleContent)>::new();
        modules_to_process.push_back((item_path, &source.module));

        while let Some((item_path, module_content)) = modules_to_process.pop_front() {
            let mut module_builder = items.build_module(item_path.clone());

            for decl in module_content.decls.iter() {
                let is_exported = decl.is_exported;

                match &decl.kind {
                    DeclKind::Item(decl) => {
                        if let Item::Module(child_module) = &decl.item {
                            match &child_module.kind {
                                ModuleKind::Inline(module_content) => {
                                    let child_path = item_path.append(decl.ident.symbol);
                                    modules_to_process.push_back((child_path, module_content));
                                }
                                ModuleKind::External => {
                                    let child_path = item_path.append(decl.ident.symbol);
                                    let child_module_paths = ModulePaths::for_child(
                                        &module_paths.child_module_dir,
                                        Symbol::get(decl.ident.symbol),
                                    )
                                    .unwrap();

                                    files_to_process.push_back((child_path, child_module_paths));
                                }
                            }
                        }

                        module_builder.add_item(is_exported, decl.ident.symbol);
                    }
                    DeclKind::Use(decl) => {
                        let item_path = decl
                            .path
                            .idents
                            .iter()
                            .map(|ident| ident.symbol)
                            .collect::<ItemPathBuf>();

                        uses_to_resolve.push_back((module_builder.id(), (is_exported, item_path)));
                    }
                }
            }
        }

        sources.push(source);
    }

    while !uses_to_resolve.is_empty() {
        uses_to_resolve = uses_to_resolve
            .iter()
            .filter(|(module_id, (is_exported, item_path))| {
                !items.insert_use_decl(*module_id, *is_exported, item_path.as_path())
            })
            .cloned()
            .collect()
    }

    items.print_final();

    Package { items, sources }
}

fn parse_source_file(item_path: ItemPathBuf, module_paths: ModulePaths) -> SourceFile {
    let content = std::fs::read_to_string(&module_paths.module_path).unwrap();
    let mut line_offsets = LineOffsets::default();
    let mut tokenizer = Tokenizer::new(&content, &mut line_offsets);

    let token_iter =
        std::iter::repeat_with(|| tokenizer.next_token()).filter(|token| token.kind.is_lang_part());
    let mut parser = Parser::new(token_iter);
    let module = parser.parse_module_file().unwrap();

    SourceFile {
        item_path,
        module_path: module_paths.module_path,
        child_module_dir: module_paths.child_module_dir,
        content,
        line_offsets,
        module,
    }
}
