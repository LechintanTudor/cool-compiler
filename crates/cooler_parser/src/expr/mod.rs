mod block_expr;
mod literal_expr;

pub use self::block_expr::*;
pub use self::literal_expr::*;

use crate::{Ident, ParseResult, Parser};
use cool_collections::define_index_newtype;
use cool_derive::Section;
use cool_lexer::{tk, TokenKind};
use derive_more::From;

define_index_newtype!(ExprId);

#[derive(Clone, Section, From, Debug)]
pub enum Expr {
    Block(BlockExpr),
    Ident(Ident),
    Literal(LiteralExpr),
}

impl Expr {
    #[inline]
    #[must_use]
    pub fn is_promotable_to_stmt(&self) -> bool {
        matches!(self, Self::Block(_))
    }
}

impl Parser<'_> {
    pub fn parse_expr(&mut self) -> ParseResult<ExprId> {
        let expr = match self.peek().kind {
            TokenKind::Ident(_) | tk::kw_crate | tk::kw_super | tk::kw_self => {
                let ident = self.parse_path_ident()?;

                if matches!(self.peek().kind, TokenKind::Literal(_)) {
                    self.continue_parse_literal_expr(ident)?
                } else {
                    self.data.exprs.push(ident.into())
                }
            }
            TokenKind::Literal(_) => self.parse_literal_expr()?,
            tk::open_brace => self.parse_block_expr()?,
            _ => todo!(),
        };

        Ok(expr)
    }

    pub fn parse_const_expr(&mut self) -> ParseResult<ExprId> {
        todo!()
    }
}
