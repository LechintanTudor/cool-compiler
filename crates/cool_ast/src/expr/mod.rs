mod block_expr;
mod fn_call_expr;
mod ident_expr;
mod literal_expr;
mod paren_expr;

pub use self::block_expr::*;
pub use self::fn_call_expr::*;
pub use self::ident_expr::*;
pub use self::literal_expr::*;
pub use self::paren_expr::*;
use crate::{AstGenerator, Unify};
use cool_parser::Expr;
use cool_resolve::expr_ty::{ExprId, ExprTyUnifier};
use cool_resolve::resolve::ScopeId;
use cool_resolve::ty::TyTable;

pub trait GenericExprAst: Unify {
    fn id(&self) -> ExprId;
}

#[derive(Clone, Debug)]
pub enum ExprAst {
    Block(BlockExprAst),
    Ident(IdentExprAst),
    Literal(LiteralExprAst),
    Paren(ParenExprAst),
    FnCall(FnCallExprAst),
}

impl Unify for ExprAst {
    fn unify(&self, unifier: &mut ExprTyUnifier, tys: &mut TyTable) {
        match self {
            Self::Block(e) => e.unify(unifier, tys),
            Self::Ident(e) => e.unify(unifier, tys),
            Self::Literal(e) => e.unify(unifier, tys),
            Self::Paren(e) => e.unify(unifier, tys),
            Self::FnCall(e) => e.unify(unifier, tys),
        }
    }
}

impl GenericExprAst for ExprAst {
    fn id(&self) -> ExprId {
        match self {
            Self::Block(e) => e.id(),
            Self::Ident(e) => e.id(),
            Self::Literal(e) => e.id(),
            Self::Paren(e) => e.id(),
            Self::FnCall(e) => e.id(),
        }
    }
}

impl AstGenerator<'_> {
    pub fn gen_expr(&mut self, scope_id: ScopeId, expr: &Expr) -> ExprAst {
        match expr {
            Expr::Block(e) => ExprAst::Block(self.gen_block_expr(scope_id, e)),
            Expr::Ident(e) => ExprAst::Ident(self.gen_ident_expr(scope_id, e)),
            Expr::Literal(e) => ExprAst::Literal(self.gen_literal_expr(e)),
            Expr::Paren(e) => ExprAst::Paren(self.gen_paren_expr(scope_id, e)),
            Expr::FnCall(e) => ExprAst::FnCall(self.gen_fn_call_expr(scope_id, e)),
            _ => todo!(),
        }
    }
}
