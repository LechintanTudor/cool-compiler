use crate::Ty;
use cool_derive::Section;
use cool_span::Span;

#[derive(Clone, Section, Debug)]
pub struct TupleTy {
    pub span: Span,
    pub elems: Vec<Ty>,
    pub has_trailing_comma: bool,
}
