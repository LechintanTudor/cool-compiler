use crate::lexer::{Literal, Token, TokenKind};
use crate::parser::{Parser, ParseResult, UnexpectedToken};
use crate::utils::Span;

#[derive(Clone, Debug)]
pub enum ExprKind {
    Literal(Literal), 
}

#[derive(Clone, Debug)]
pub struct Expr {
    pub span: Span, 
    pub kind: ExprKind,
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn parse_expr(&mut self) -> ParseResult<Expr> {
        let start_token = self.bump();

        let TokenKind::Literal(literal) = start_token.kind else {
            return Err(UnexpectedToken {
                found: start_token,
                expected: &[],
            })?;
        };
        
        Ok(Expr {
            span: start_token.span,
            kind: ExprKind::Literal(literal),
        })
    }
}
