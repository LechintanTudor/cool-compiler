use crate::{Ident, ParseTree};
use cool_span::Span;

#[derive(Clone, Copy, Debug)]
pub struct AssignPattern {
    pub ident: Ident,
}

impl ParseTree for AssignPattern {
    #[inline]
    fn span(&self) -> Span {
        self.ident.span
    }
}
