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
use cool_resolve::resolve::ScopeId;

#[derive(Clone, Debug)]
pub enum ExprAst {
    Block(BlockExprAst),
    Ident(IdentExprAst),
    Literal(LiteralExprAst),
    Paren(ParenExprAst),
    FnCall(FnCallExprAst),
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
