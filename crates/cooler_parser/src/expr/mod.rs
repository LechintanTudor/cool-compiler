mod access_expr;
mod block_expr;
mod fn_expr;
mod for_expr;
mod if_expr;
mod literal_expr;
mod loop_expr;
mod struct_expr;
mod tuple_expr;
mod while_expr;

pub use self::access_expr::*;
pub use self::block_expr::*;
pub use self::fn_expr::*;
pub use self::for_expr::*;
pub use self::if_expr::*;
pub use self::literal_expr::*;
pub use self::loop_expr::*;
pub use self::struct_expr::*;
pub use self::tuple_expr::*;
pub use self::while_expr::*;

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
    For(ForExpr),
    Fn(FnExpr),
    Ident(Ident),
    If(IfExpr),
    Literal(LiteralExpr),
    Loop(LoopExpr),
    Paren(ParenExpr),
    Struct(StructExpr),
    Tuple(TupleExpr),
    While(WhileExpr),
}

impl Expr {
    #[inline]
    #[must_use]
    pub fn is_promotable_to_stmt(&self) -> bool {
        matches!(
            self,
            Self::Block(_) | Self::For(_) | Self::If(_) | Self::Loop(_) | Self::While(_),
        )
    }
}

impl Parser<'_> {
    #[inline]
    pub fn parse_expr(&mut self) -> ParseResult<ExprId> {
        self.parse_expr_full(true)
    }

    #[inline]
    pub fn parse_non_struct_expr(&mut self) -> ParseResult<ExprId> {
        self.parse_expr_full(false)
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
            tk::kw_for => self.parse_for_expr()?,
            tk::kw_if => self.parse_if_expr()?,
            tk::kw_loop => self.parse_loop_expr()?,
            tk::kw_while => self.parse_while_expr()?,
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
