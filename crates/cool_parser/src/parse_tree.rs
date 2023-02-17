use cool_span::Span;

pub trait ParseTree {
    fn span(&self) -> Span;

    #[inline]
    fn span_to(&self, other: &dyn ParseTree) -> Span {
        self.span().to(other.span())
    }
}
