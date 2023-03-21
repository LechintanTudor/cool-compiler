use crate::item::item_decl::ItemDeclAst;
use crate::{AstGenerator, ItemAst};
use cool_parser::{Item, ModuleContent};
use cool_resolve::ModuleId;

#[derive(Clone, Debug)]
pub struct ModuleItemAst {
    pub decls: Vec<ItemDeclAst>,
}

impl AstGenerator<'_> {
    pub fn generate_module(
        &mut self,
        module_id: ModuleId,
        module: &ModuleContent,
    ) -> ModuleItemAst {
        let mut decls = Vec::<ItemDeclAst>::new();

        for decl in module
            .decls
            .iter()
            .flat_map(|decl| decl.kind.as_item_decl())
        {
            let item: ItemAst = match &decl.item {
                Item::Fn(fn_item) => self.generate_fn(module_id, fn_item).into(),
                _ => todo!(),
            };

            decls.push(ItemDeclAst {
                symbol: decl.ident.symbol,
                item,
            });
        }

        ModuleItemAst { decls }
    }
}
