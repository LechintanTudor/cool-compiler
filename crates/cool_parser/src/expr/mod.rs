mod access_expr;
mod array_expr;
mod array_repeat_expr;
mod binary_expr;
mod block_expr;
mod deref_expr;
mod fn_call_expr;
mod fn_expr;
mod if_expr;
mod index_expr;
mod literal_expr;
mod loop_expr;
mod paren_expr;
mod range_expr;
mod struct_expr;
mod tuple_expr;
mod unary_expr;
mod utils;
mod while_expr;

pub use self::access_expr::*;
pub use self::array_expr::*;
pub use self::array_repeat_expr::*;
pub use self::binary_expr::*;
pub use self::block_expr::*;
pub use self::deref_expr::*;
pub use self::fn_call_expr::*;
pub use self::fn_expr::*;
pub use self::if_expr::*;
pub use self::index_expr::*;
pub use self::literal_expr::*;
pub use self::loop_expr::*;
pub use self::paren_expr::*;
pub use self::range_expr::*;
pub use self::struct_expr::*;
pub use self::tuple_expr::*;
pub use self::unary_expr::*;
pub use self::while_expr::*;

use crate::{BinaryOp, Ident, ParseResult, Parser};
use cool_derive::Section;
use cool_lexer::{tk, TokenKind};
use cool_span::Section;
use derive_more::From;

#[derive(Clone, From, Section, Debug)]
pub enum Expr {
    Access(AccessExpr),
    Array(ArrayExpr),
    ArrayRepeat(ArrayRepeatExpr),
    Binary(BinaryExpr),
    Block(BlockExpr),
    Deref(DerefExpr),
    Fn(FnExpr),
    FnCall(FnCallExpr),
    Ident(Ident),
    If(IfExpr),
    Index(IndexExpr),
    Literal(LiteralExpr),
    Loop(LoopExpr),
    Paren(ParenExpr),
    Range(RangeExpr),
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
            Self::Block(_) | Self::If(_) | Self::Loop(_) | Self::While(_)
        )
    }
}

#[derive(Clone, From, Debug)]
enum ExprPart {
    Expr(Expr),
    BinOp(BinaryOp),
}

impl ExprPart {
    #[inline]
    pub fn into_expr(self) -> Expr {
        match self {
            Self::Expr(expr) => expr,
            _ => panic!("Part is not an expression"),
        }
    }
}

impl Parser<'_> {
    #[inline]
    pub fn parse_expr(&mut self) -> ParseResult<Expr> {
        self.parse_expr_full(true)
    }

    #[inline]
    pub fn parse_non_struct_expr(&mut self) -> ParseResult<Expr> {
        self.parse_expr_full(false)
    }

    fn parse_expr_full(&mut self, allow_struct: bool) -> ParseResult<Expr> {
        let expr = self.parse_primary_expr(allow_struct)?;

        let (first_binary_op, second_expr) = match BinaryOp::try_from(self.peek().kind) {
            Ok(binary_op) => {
                self.bump();
                (binary_op, self.parse_primary_expr(allow_struct)?)
            }
            Err(_) => return Ok(expr),
        };

        let mut parts: Vec<ExprPart> = vec![expr.into(), second_expr.into()];
        let mut binary_ops = vec![first_binary_op];

        while let Ok(binary_op) = BinaryOp::try_from(self.peek().kind) {
            self.bump();

            while let Some(&last_binary_op) = binary_ops.last() {
                if last_binary_op.precedence() < binary_op.precedence() {
                    break;
                }

                parts.push(last_binary_op.into());
                binary_ops.pop();
            }

            binary_ops.push(binary_op);
            parts.push(self.parse_primary_expr(allow_struct)?.into());
        }

        while let Some(binary_op) = binary_ops.pop() {
            parts.push(binary_op.into());
        }

        let mut part_stack = Vec::<ExprPart>::new();

        for part in parts {
            match part {
                ExprPart::Expr(_) => {
                    part_stack.push(part);
                }
                ExprPart::BinOp(binary_op) => {
                    let rhs = part_stack.pop().unwrap().into_expr();
                    let lhs = part_stack.pop().unwrap().into_expr();

                    part_stack.push(
                        Expr::Binary(BinaryExpr {
                            op: binary_op,
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                        })
                        .into(),
                    )
                }
            }
        }

        debug_assert!(part_stack.len() == 1);
        Ok(part_stack.pop().unwrap().into_expr())
    }

    fn parse_primary_expr(&mut self, allow_struct: bool) -> ParseResult<Expr> {
        let peeked_token = self.peek();

        let mut expr = match peeked_token.kind {
            TokenKind::Ident(_) | tk::kw_crate | tk::kw_super | tk::kw_self => {
                let ident = self.parse_path_ident()?;

                if matches!(self.peek_any().kind, TokenKind::Literal(_)) {
                    self.continue_parse_literal_expr(ident)?.into()
                } else {
                    ident.into()
                }
            }
            TokenKind::Literal(_) => self.parse_literal_expr()?.into(),
            tk::minus | tk::not | tk::and => {
                let unary_op = self.parse_unary_op()?;
                let expr = self.parse_expr()?;

                UnaryExpr {
                    span: peeked_token.span.to(expr.span()),
                    op: unary_op,
                    expr: Box::new(expr),
                }
                .into()
            }
            tk::open_paren => self.parse_paren_or_tuple_expr()?,
            tk::open_brace => self.parse_block_expr()?.into(),
            tk::open_bracket => self.parse_array_or_array_repeat_expr()?,
            tk::kw_extern | tk::kw_fn => self.parse_fn_expr()?.into(),
            tk::kw_if => self.parse_if_expr()?.into(),
            tk::kw_loop => self.parse_loop_expr()?.into(),
            tk::kw_while => self.parse_while_expr()?.into(),
            token => todo!("{:?}", token),
        };

        loop {
            expr = match expr {
                Expr::Access(_) | Expr::Ident(_) => {
                    match self.peek().kind {
                        tk::open_paren => self.continue_parse_fn_call_expr(expr)?.into(),
                        tk::open_brace if allow_struct => {
                            self.continue_parse_struct_expr(expr)?.into()
                        }
                        tk::open_bracket => self.continue_parse_index_or_range_expr(expr)?,
                        tk::dot => self.continue_parse_access_or_deref_expr(expr)?,
                        _ => break,
                    }
                }
                _ => break,
            };
        }

        Ok(expr)
    }
}
