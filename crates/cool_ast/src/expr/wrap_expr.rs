use crate::{AstGenerator, ExprAst};
use cool_resolve::{ExprId, TyId};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct WrapExprAst {
    pub expr_id: ExprId,
    pub inner: Box<ExprAst>,
}

impl Section for WrapExprAst {
    #[inline]
    fn span(&self) -> Span {
        self.inner.span()
    }
}

impl AstGenerator<'_> {
    pub fn continue_gen_wrap_expr(&mut self, expr: ExprAst, ty_id: TyId) -> WrapExprAst {
        WrapExprAst {
            expr_id: self.context.add_rvalue_expr(ty_id),
            inner: Box::new(expr),
        }
    }
}
