use crate::Span;

pub trait Section {
    #[must_use]
    fn span(&self) -> Span;
}
