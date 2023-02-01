use crate::lexer::{sep, Token, TokenKind};
use crate::parser::{FnExpr, ParseResult, Parser, UnexpectedToken};
use crate::symbol::Symbol;
use crate::utils::Span;

#[derive(Clone, Debug)]
pub struct StaticDecl {
    pub span: Span,
    pub ident: Symbol,
    pub expr: FnExpr,
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn parse_static_decl(&mut self) -> ParseResult<StaticDecl> {
        let start_token = self.bump();

        let ident = match start_token.kind {
            TokenKind::Ident(ident) => ident,
            _ => {
                return Err(UnexpectedToken {
                    found: start_token,
                    expected: &[],
                })?;
            }
        };

        self.bump_expect(&[sep::COLON])?;
        self.bump_expect(&[sep::COLON])?;

        let expr = self.parse_fn()?;

        let end_token = self.bump_expect(&[sep::SEMI])?;
        let span = Span::from_start_and_end_spans(start_token.span, end_token.span);

        Ok(StaticDecl { span, ident, expr })
    }
}
