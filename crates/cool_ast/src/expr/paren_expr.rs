use crate::expr::ExprAst;
use crate::AstGenerator;
use cool_parser::ParenExpr;
use cool_resolve::ScopeId;

#[derive(Clone, Debug)]
pub struct ParenExprAst {
    pub inner: Box<ExprAst>,
}

impl AstGenerator<'_> {
    pub fn generate_paren_expr(&mut self, scope_id: ScopeId, expr: &ParenExpr) -> ParenExprAst {
        ParenExprAst {
            inner: Box::new(self.generate_expr(scope_id, &expr.inner)),
        }
    }
}
