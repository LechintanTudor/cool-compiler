use crate::error::{ParseResult, UnexpectedToken};
use crate::parser::Parser;
use cool_lexer::tokens::{tk, Literal, Token, TokenKind};
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct LiteralExpr {
    pub span: Span,
    pub literal: Literal,
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn parse_literal_expr(&mut self) -> ParseResult<LiteralExpr> {
        let start_token = self.bump();

        let TokenKind::Literal(literal) = start_token.kind else {
            return Err(UnexpectedToken {
                found: start_token,
                expected: &[tk::ANY_LITERAL],
            })?;
        };

        Ok(LiteralExpr {
            span: start_token.span,
            literal,
        })
    }
}
