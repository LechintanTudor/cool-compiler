use crate::utils::Span;

pub trait Ast {
    fn span(&self) -> Span;
}
