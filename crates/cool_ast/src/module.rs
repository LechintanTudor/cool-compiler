use crate::{AstGenerator, ItemAst, ItemDeclAst};
use cool_parser::item::{Item, ModuleContent, ModuleKind};

#[derive(Clone, Debug)]
pub struct ModuleAst {
    pub decls: Vec<ItemDeclAst>,
}

impl AstGenerator<'_> {
    pub fn generate_module(&mut self, module: &ModuleContent) -> ModuleAst {
        let mut decls = Vec::<ItemDeclAst>::new();

        for decl in module
            .decls
            .iter()
            .flat_map(|decl| decl.kind.as_item_decl())
        {
            let item: ItemAst = match &decl.item {
                Item::Fn(fn_item) => self.generate_fn(fn_item).into(),
                Item::Module(module_item) => match &module_item.kind {
                    ModuleKind::Inline(module_item) => self.generate_module(module_item).into(),
                    _ => continue,
                },
            };

            decls.push(ItemDeclAst {
                symbol: decl.ident.symbol,
                item,
            });
        }

        ModuleAst { decls }
    }
}
