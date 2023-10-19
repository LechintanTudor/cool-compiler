mod block_expr;
mod fn_expr;
mod ident_expr;
mod literal_expr;

pub use self::block_expr::*;
pub use self::fn_expr::*;
pub use self::ident_expr::*;
pub use self::literal_expr::*;

use crate::{ParseResult, Parser};
use cool_derive::Section;
use cool_lexer::{tk, TokenKind};
use derive_more::From;

#[derive(Clone, From, Section, Debug)]
pub enum Expr {
    Block(BlockExpr),
    Fn(FnExpr),
    Ident(IdentExpr),
    Literal(LiteralExpr),
}

impl Parser<'_> {
    #[inline]
    pub fn parse_expr(&mut self) -> ParseResult<Expr> {
        self.parse_primary_expr()
    }

    fn parse_primary_expr(&mut self) -> ParseResult<Expr> {
        let expr = match self.peek().kind {
            TokenKind::Ident(_) => {
                let ident_expr = self.parse_ident_expr()?;

                if matches!(self.peek_any().kind, TokenKind::Literal(_)) {
                    self.continue_parse_literal_expr(ident_expr.ident)?.into()
                } else {
                    ident_expr.into()
                }
            }
            TokenKind::Literal(_) => self.parse_literal_expr()?.into(),
            tk::open_brace => self.parse_block_expr()?.into(),
            tk::kw_extern | tk::kw_fn => self.parse_fn_expr()?.into(),
            token => todo!("{:?}", token),
        };

        Ok(expr)
    }
}
