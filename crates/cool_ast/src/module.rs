use crate::{AstGenerator, ItemAst};
use cool_parser::item::{DeclKind, Item, ItemDecl, ModuleContent};

#[derive(Clone, Debug)]
pub struct ModuleAst {
    pub items: Vec<ItemAst>,
}

impl AstGenerator<'_> {
    pub fn generate_module(&mut self, module: &ModuleContent) -> ModuleAst {
        let mut items = Vec::<ItemAst>::new();

        for decl in module.decls.iter() {
            match &decl.kind {
                DeclKind::Item(ItemDecl {
                    item: Item::Fn(fn_item),
                    ..
                }) => items.push(ItemAst::Fn(self.generate_fn(fn_item))),
                _ => (),
            }
        }

        ModuleAst { items }
    }
}
