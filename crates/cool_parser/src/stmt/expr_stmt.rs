use crate::Expr;
use cool_derive::Section;
use cool_span::Span;

#[derive(Clone, Section, Debug)]
pub struct ExprStmt {
    pub span: Span,
    pub expr: Box<Expr>,
    pub has_semicolon: bool,
}
