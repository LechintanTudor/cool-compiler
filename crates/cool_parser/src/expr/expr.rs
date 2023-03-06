use crate::expr::LiteralExpr;
use crate::path::SymbolPath;
use crate::{ParseResult, Parser, UnexpectedToken};
use cool_lexer::tokens::{tk, Token, TokenKind};

#[derive(Clone, Debug)]
pub enum Expr {
    Literal(LiteralExpr),
    Path(SymbolPath),
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn parse_expr(&mut self) -> ParseResult<Expr> {
        let expr = match self.peek().kind {
            TokenKind::Literal(_) => Expr::Literal(self.parse_literal_expr()?),
            TokenKind::Ident(_) => Expr::Path(self.parse_import_path()?),
            _ => {
                return Err(UnexpectedToken {
                    found: self.peek(),
                    expected: &[tk::ANY_LITERAL, tk::ANY_IDENT],
                })?
            }
        };

        Ok(expr)
    }
}
