use crate::Span;

pub trait Section {
    fn span(&self) -> Span;
}
