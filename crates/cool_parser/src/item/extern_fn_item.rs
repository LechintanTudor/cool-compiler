use crate::FnPrototype;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct ExternFnItem {
    pub prototype: FnPrototype,
}

impl Section for ExternFnItem {
    #[inline]
    fn span(&self) -> Span {
        self.prototype.span
    }
}
