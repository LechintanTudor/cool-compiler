mod block_expr;
mod ident_expr;

pub use self::block_expr::*;
pub use self::ident_expr::*;

use crate::{ParseResult, Parser};
use cool_derive::Section;
use cool_lexer::{tk, TokenKind};
use derive_more::From;

#[derive(Clone, From, Section, Debug)]
pub enum Expr {
    Block(BlockExpr),
    Ident(IdentExpr),
}

impl Parser<'_> {
    #[inline]
    pub fn parse_expr(&mut self) -> ParseResult<Expr> {
        self.parse_primary_expr()
    }

    fn parse_primary_expr(&mut self) -> ParseResult<Expr> {
        let expr = match self.peek().kind {
            TokenKind::Ident(_) => self.parse_ident_expr()?.into(),
            tk::open_brace => self.parse_block_expr()?.into(),
            _ => todo!(),
        };

        Ok(expr)
    }
}
