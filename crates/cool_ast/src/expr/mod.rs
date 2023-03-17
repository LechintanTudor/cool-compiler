mod block_expr;
mod ident_expr;
mod literal_expr;
mod paren_expr;

pub use self::block_expr::*;
pub use self::ident_expr::*;
pub use self::literal_expr::*;
pub use self::paren_expr::*;
use cool_resolve::binding::BindingTable;
use cool_resolve::ty::TyId;

pub trait GenericExprAst {
    fn ty_id(&self, bindings: &BindingTable) -> Option<TyId>;
}

#[derive(Clone, Debug)]
pub enum ExprAst {
    Block(BlockExprAst),
    Ident(IdentExprAst),
    Literal(LiteralExprAst),
    Paren(ParenExprAst),
}

impl GenericExprAst for ExprAst {
    fn ty_id(&self, bindings: &BindingTable) -> Option<TyId> {
        match self {
            Self::Block(e) => e.ty_id(bindings),
            Self::Ident(e) => e.ty_id(bindings),
            Self::Literal(e) => e.ty_id(bindings),
            Self::Paren(e) => e.ty_id(bindings),
        }
    }
}
