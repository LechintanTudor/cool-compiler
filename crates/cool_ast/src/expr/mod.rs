mod block_expr;
mod ident_expr;
mod literal_expr;
mod paren_expr;

pub use self::block_expr::*;
pub use self::ident_expr::*;
pub use self::literal_expr::*;
pub use self::paren_expr::*;
use crate::AstGenerator;
use cool_parser::Expr;
use cool_resolve::binding::{BindingTable, FrameId};
use cool_resolve::item::ItemId;
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

impl AstGenerator<'_> {
    pub fn generate_expr(
        &mut self,
        module_id: ItemId,
        parent_id: Option<FrameId>,
        expr: &Expr,
    ) -> ExprAst {
        match expr {
            Expr::Block(e) => ExprAst::Block(self.generate_block_expr(module_id, parent_id, e)),
            Expr::Ident(e) => ExprAst::Ident(self.generate_ident_expr(module_id, parent_id, e)),
            Expr::Literal(e) => ExprAst::Literal(self.generate_literal_expr(e)),
            Expr::Paren(e) => ExprAst::Paren(self.generate_paren_expr(module_id, parent_id, e)),
            _ => todo!(),
        }
    }
}
