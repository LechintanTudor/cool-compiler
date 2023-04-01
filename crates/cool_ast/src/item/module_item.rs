use crate::item::item_decl::ItemDeclAst;
use crate::{AstGenerator, ItemAst, Unify};
use cool_parser::{Item, ModuleContent};
use cool_resolve::resolve::ModuleId;

#[derive(Clone, Debug)]
pub struct ModuleItemAst {
    pub decls: Vec<ItemDeclAst>,
}

impl AstGenerator<'_> {
    pub fn gen_module(&mut self, module_id: ModuleId, module: &ModuleContent) -> ModuleItemAst {
        let mut decls = Vec::<ItemDeclAst>::new();

        for decl in module
            .decls
            .iter()
            .flat_map(|decl| decl.kind.as_item_decl())
        {
            let item: ItemAst = match &decl.item {
                Item::Fn(fn_item) => {
                    let fn_ast = self.gen_fn(module_id, fn_item);

                    fn_ast.unify(self);

                    println!("\n\n\n\n{:#?}", self.unification.unify());

                    fn_ast.into()
                }
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
