use crate::paths::ModulePaths;
use crate::SourceFile;
use cool_lexer::lexer::{LineOffsets, Tokenizer};
use cool_lexer::symbols::Symbol;
use cool_parser::item::{DeclKind, Item, ModuleContent, ModuleKind};
use cool_parser::Parser;
use cool_resolve::item::{ItemPathBuf, ItemTable};
use std::collections::VecDeque;
use std::path::Path;

// TODO:
// Get a full parse tree (add support for multi-file packages)
// Then walk the tree to create the item table
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

    while let Some((item_path, module_paths)) = files_to_process.pop_front() {
        let source = parse_source_file(item_path.clone(), module_paths.clone());

        let mut modules_to_process = VecDeque::<(ItemPathBuf, &ModuleContent)>::new();
        modules_to_process.push_back((item_path, &source.module));

        while let Some((item_path, module_content)) = modules_to_process.pop_front() {
            let mut module_builder = items.build_module(item_path.clone());

            for decl in module_content.decls.iter() {
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

                        module_builder.add_item(decl.ident.symbol);
                    }
                    _ => (),
                }
            }
        }

        sources.push(source);
    }

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
