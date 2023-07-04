use cool_resolve::ExprId;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct UnitExprAst {
    pub span: Span,
    pub expr_id: ExprId,
}

impl Section for UnitExprAst {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}
