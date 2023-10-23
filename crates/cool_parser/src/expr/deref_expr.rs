use crate::Expr;
use cool_derive::Section;
use cool_span::Span;

#[derive(Clone, Section, Debug)]
pub struct DerefExpr {
    pub span: Span,
    pub base: Box<Expr>,
}
