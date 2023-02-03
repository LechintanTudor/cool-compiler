use crate::lexer::{Literal, Token, TokenKind};
use crate::parser::{ParseResult, Parser, UnexpectedToken};
use crate::utils::Span;

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
                expected: &[],
            })?;
        };

        Ok(LiteralExpr {
            span: start_token.span,
            literal,
        })
    }
}
