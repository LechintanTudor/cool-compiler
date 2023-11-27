use cool_lexer::Symbol;
use cool_span::Span;
use derive_more::Display;

#[derive(Clone, Copy, Debug, Display)]
#[display("{}", self.symbol)]
pub struct Ident {
    pub span: Span,
    pub symbol: Symbol,
}
