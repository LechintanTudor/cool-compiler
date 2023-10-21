use crate::{Expr, UnaryOp};
use cool_derive::Section;
use cool_span::Span;

#[derive(Clone, Section, Debug)]
pub struct UnaryExpr {
    pub span: Span,
    pub op: UnaryOp,
    pub expr: Box<Expr>,
}
