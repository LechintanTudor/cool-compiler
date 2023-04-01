use crate::expr::ExprAst;
use crate::{AstGenerator, Unify};

#[derive(Clone, Debug)]
pub struct ExprStmtAst {
    pub expr: ExprAst,
}

impl Unify for ExprStmtAst {
    #[inline]
    fn unify(&self, gen: &mut AstGenerator) {
        self.expr.unify(gen)
    }
}
