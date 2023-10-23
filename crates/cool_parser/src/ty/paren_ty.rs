use crate::Ty;
use cool_derive::Section;
use cool_span::Span;

#[derive(Clone, Section, Debug)]
pub struct ParenTy {
    pub span: Span,
    pub ty: Box<Ty>,
}
