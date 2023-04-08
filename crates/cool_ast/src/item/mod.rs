mod const_item;
mod item_decl;
mod module_item;

pub use self::const_item::*;
pub use self::item_decl::*;
pub use self::module_item::*;
use crate::{AstGenerator, AstResult, ResolveAst};
use cool_resolve::TyId;

#[derive(Clone, Debug)]
pub enum ItemAst {
    Module(ModuleItemAst),
    Const(ConstItemAst),
}

impl ResolveAst for ItemAst {
    fn resolve(&self, ast: &mut AstGenerator, expected_ty: TyId) -> AstResult<TyId> {
        match self {
            Self::Module(item) => item.resolve(ast, expected_ty),
            Self::Const(item) => item.resolve(ast, expected_ty),
        }
    }
}

impl From<ModuleItemAst> for ItemAst {
    #[inline]
    fn from(module_item: ModuleItemAst) -> Self {
        Self::Module(module_item)
    }
}

impl From<ConstItemAst> for ItemAst {
    #[inline]
    fn from(const_item: ConstItemAst) -> Self {
        Self::Const(const_item)
    }
}
