mod access_expr;
mod array_expr;
mod as_expr;
mod binary_expr;
mod block_expr;
mod fn_call_expr;
mod fn_expr;
mod for_expr;
mod if_expr;
mod literal_expr;
mod loop_expr;
mod struct_expr;
mod tuple_expr;
mod unary_expr;
mod while_expr;

pub use self::access_expr::*;
pub use self::array_expr::*;
pub use self::as_expr::*;
pub use self::binary_expr::*;
pub use self::block_expr::*;
pub use self::fn_call_expr::*;
pub use self::fn_expr::*;
pub use self::for_expr::*;
pub use self::if_expr::*;
pub use self::literal_expr::*;
pub use self::loop_expr::*;
pub use self::struct_expr::*;
pub use self::tuple_expr::*;
pub use self::unary_expr::*;
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
    Array(ArrayExpr),
    ArrayRepeat(ArrayRepeatExpr),
    As(AsExpr),
    Block(BlockExpr),
    Binary(BinaryExpr),
    Deref(DerefExpr),
    Fn(FnExpr),
    FnCall(FnCallExpr),
    For(ForExpr),
    Ident(Ident),
    If(IfExpr),
    Literal(LiteralExpr),
    Loop(LoopExpr),
    Paren(ParenExpr),
    Struct(StructExpr),
    Tuple(TupleExpr),
    Unary(UnaryExpr),
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

    fn parse_primary_expr(&mut self, allow_struct: bool) -> ParseResult<ExprId> {
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
            tk::open_bracket => self.parse_array_or_array_repeat_expr()?,
            tk::open_paren => self.parse_paren_or_tuple_expr()?,
            tk::and | tk::minus | tk::not => self.parse_unary_expr()?,
            token => todo!("Cannot parse expr with: {token}"),
        };

        loop {
            let can_parse_struct = allow_struct
                && matches!(
                    &self[expr],
                    Expr::Access(_) | Expr::Ident(_) | Expr::Paren(_)
                );

            expr = match self.peek().kind {
                tk::kw_as => self.continue_parse_as_expr(expr)?,
                tk::dot => self.continue_parse_access_or_deref_expr(expr)?,
                tk::open_brace if can_parse_struct => self.continue_parse_struct_expr(expr)?,
                tk::open_paren => self.continue_parse_fn_call_expr(expr)?,
                _ => break,
            };
        }

        Ok(expr)
    }

    pub fn parse_const_expr(&mut self) -> ParseResult<ExprId> {
        todo!()
    }
}
