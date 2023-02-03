use crate::lexer::Token;
use crate::parser::{LiteralExpr, ParseResult, Parser};

#[derive(Clone, Debug)]
pub enum Expr {
    Literal(LiteralExpr),
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn parse_expr(&mut self) -> ParseResult<Expr> {
        Ok(Expr::Literal(self.parse_literal_expr()?))
    }
}
