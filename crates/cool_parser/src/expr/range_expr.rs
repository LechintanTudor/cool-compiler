use crate::Expr;
use cool_derive::Section;
use cool_span::Span;

#[derive(Clone, Debug)]
pub enum RangeExprKind {
    Full,
    From(Box<Expr>),
    To(Box<Expr>),
    FromTo(Box<(Expr, Expr)>),
}

#[derive(Clone, Section, Debug)]
pub struct RangeExpr {
    pub span: Span,
    pub base: Box<Expr>,
    pub is_mutable: bool,
    pub kind: RangeExprKind,
}
