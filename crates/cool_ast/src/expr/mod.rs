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
use crate::AstGenerator;
use cool_parser::Expr;
use cool_resolve::expr_ty::ExprId;
use cool_resolve::resolve::ScopeId;
use cool_resolve::ty::tys;

#[derive(Clone, Debug)]
pub struct ExprAst {
    pub id: ExprId,
    pub kind: ExprKindAst,
}

#[derive(Clone, Debug)]
pub enum ExprKindAst {
    Block(BlockExprAst),
    Ident(IdentExprAst),
    Literal(LiteralExprAst),
    Paren(ParenExprAst),
    FnCall(FnCallExprAst),
}

impl AstGenerator<'_> {
    pub fn gen_expr(&mut self, scope_id: ScopeId, expr: &Expr) -> ExprAst {
        let kind = match expr {
            Expr::Block(e) => ExprKindAst::Block(self.gen_block_expr(scope_id, e)),
            Expr::Ident(e) => ExprKindAst::Ident(self.gen_ident_expr(scope_id, e)),
            Expr::Literal(e) => ExprKindAst::Literal(self.gen_literal_expr(e)),
            Expr::Paren(e) => ExprKindAst::Paren(self.gen_paren_expr(scope_id, e)),
            Expr::FnCall(e) => ExprKindAst::FnCall(self.gen_fn_call_expr(scope_id, e)),
            _ => todo!(),
        };

        let id = self.expr_tys.add_expr(tys::UNIT);

        ExprAst { id, kind }
    }
}
