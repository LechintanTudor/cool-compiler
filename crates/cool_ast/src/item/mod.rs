mod fn_item;
mod item_decl;
mod module_item;

pub use self::fn_item::*;
pub use self::item_decl::*;
pub use self::module_item::*;

#[derive(Clone, Debug)]
pub enum ItemAst {
    Fn(FnItemAst),
    Module(ModuleItemAst),
}

impl From<FnItemAst> for ItemAst {
    #[inline]
    fn from(fn_ast: FnItemAst) -> Self {
        Self::Fn(fn_ast)
    }
}

impl From<ModuleItemAst> for ItemAst {
    #[inline]
    fn from(module_ast: ModuleItemAst) -> Self {
        Self::Module(module_ast)
    }
}
