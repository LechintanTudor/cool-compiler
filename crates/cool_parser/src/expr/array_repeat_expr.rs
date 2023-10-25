use crate::{ArrayLen, Expr};
use cool_derive::Section;
use cool_span::Span;

#[derive(Clone, Section, Debug)]
pub struct ArrayRepeatExpr {
    pub span: Span,
    pub value: Box<Expr>,
    pub len: ArrayLen,
}
