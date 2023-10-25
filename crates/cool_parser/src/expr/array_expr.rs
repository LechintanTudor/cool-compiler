use crate::Expr;
use cool_derive::Section;
use cool_span::Span;

#[derive(Clone, Section, Debug)]
pub struct ArrayExpr {
    pub span: Span,
    pub values: Vec<Expr>,
    pub has_trailing_comma: bool,
}
