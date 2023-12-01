mod access_expr;
mod block_expr;
mod fn_expr;
mod literal_expr;
mod struct_expr;
mod tuple_expr;

pub use self::access_expr::*;
pub use self::block_expr::*;
pub use self::fn_expr::*;
pub use self::literal_expr::*;
pub use self::struct_expr::*;
pub use self::tuple_expr::*;

use crate::{Ident, ParseResult, Parser};
use cool_collections::define_index_newtype;
use cool_derive::Section;
use cool_lexer::{tk, TokenKind};
use derive_more::From;

define_index_newtype!(ExprId);

#[derive(Clone, Section, From, Debug)]
pub enum Expr {
    Access(AccessExpr),
    Block(BlockExpr),
    Deref(DerefExpr),
    Fn(FnExpr),
    Ident(Ident),
    Literal(LiteralExpr),
    Paren(ParenExpr),
    Struct(StructExpr),
    Tuple(TupleExpr),
}

impl Expr {
    #[inline]
    #[must_use]
    pub fn is_promotable_to_stmt(&self) -> bool {
        matches!(self, Self::Block(_))
    }
}

impl Parser<'_> {
    #[inline]
    pub fn parse_expr(&mut self) -> ParseResult<ExprId> {
        self.parse_expr_full(true)
    }

    pub fn parse_expr_full(&mut self, allow_struct: bool) -> ParseResult<ExprId> {
        let mut expr = match self.peek().kind {
            TokenKind::Ident(_) | tk::kw_crate | tk::kw_super | tk::kw_self => {
                let ident = self.parse_path_ident()?;

                if matches!(self.peek().kind, TokenKind::Literal(_)) {
                    self.continue_parse_literal_expr(ident)?
                } else {
                    self.add_expr(ident)
                }
            }
            TokenKind::Literal(_) => self.parse_literal_expr()?,
            tk::kw_extern | tk::kw_fn => self.parse_fn_expr()?,
            tk::open_brace => self.parse_block_expr()?,
            tk::open_paren => self.parse_paren_or_tuple_expr()?,
            token => todo!("Cannot parse expr with: {token}"),
        };

        loop {
            expr = match &self[expr] {
                Expr::Access(_) | Expr::Ident(_) => {
                    match self.peek().kind {
                        tk::dot => self.continue_parse_access_or_deref_expr(expr)?,
                        tk::open_brace if allow_struct => self.continue_parse_struct_expr(expr)?,
                        _ => break,
                    }
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    pub fn parse_const_expr(&mut self) -> ParseResult<ExprId> {
        todo!()
    }
}
