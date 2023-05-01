use cool_span::Span;

pub trait ParseTree {
    fn span(&self) -> Span;
}
