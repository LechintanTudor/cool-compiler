use crate::Expr;
use cool_derive::Section;
use cool_span::Span;

#[derive(Clone, Section, Debug)]
pub struct TupleExpr {
    pub span: Span,
    pub elems: Vec<Expr>,
    pub has_trailing_comma: bool,
}
