use crate::{FnPrototype, ParseTree};
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct ExternFnItem {
    pub prototype: FnPrototype,
}

impl ParseTree for ExternFnItem {
    #[inline]
    fn span(&self) -> Span {
        self.prototype.span
    }
}
