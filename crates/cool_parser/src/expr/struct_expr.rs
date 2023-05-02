use crate::{Expr, Ident, ParseTree};
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct StructFieldInitializer {
    pub ident: Ident,
    pub expr: Box<Expr>,
}

impl ParseTree for StructFieldInitializer {
    #[inline]
    fn span(&self) -> Span {
        self.ident.span.to(self.expr.span())
    }
}

#[derive(Clone, Debug)]
pub struct StructExpr {
    pub span: Span,
    pub base: Box<Expr>,
    pub initializers: Vec<StructFieldInitializer>,
    pub has_trailing_comma: bool,
}

impl ParseTree for StructExpr {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}
