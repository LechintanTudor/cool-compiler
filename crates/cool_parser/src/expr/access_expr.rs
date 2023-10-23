use crate::{Expr, Ident};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct AccessExpr {
    pub base: Box<Expr>,
    pub field: Ident,
}

impl Section for AccessExpr {
    #[inline]
    fn span(&self) -> Span {
        self.base.span().to(self.field.span)
    }
}
