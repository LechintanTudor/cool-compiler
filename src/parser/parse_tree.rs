use crate::utils::Span;

pub trait ParseTree {
    fn span(&self) -> Span;
}
