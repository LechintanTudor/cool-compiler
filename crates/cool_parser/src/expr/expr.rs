use crate::error::ParseResult;
use crate::expr::LiteralExpr;
use crate::parser::Parser;
use cool_lexer::tokens::Token;

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
