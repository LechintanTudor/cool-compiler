mod access_expr;
mod array_expr;
mod binary_expr;
mod block_expr;
mod deref_expr;
mod fn_call_expr;
mod fn_expr;
mod ident_expr;
mod literal_expr;
mod paren_expr;
mod return_expr;
mod subscript_expr;
mod tuple_expr;

pub use self::access_expr::*;
pub use self::array_expr::*;
pub use self::binary_expr::*;
pub use self::block_expr::*;
pub use self::deref_expr::*;
pub use self::fn_call_expr::*;
pub use self::fn_expr::*;
pub use self::ident_expr::*;
pub use self::literal_expr::*;
pub use self::paren_expr::*;
pub use self::return_expr::*;
pub use self::subscript_expr::*;
pub use self::tuple_expr::*;
use crate::{BinOp, Ident, ParseResult, ParseTree, Parser};
use cool_lexer::tokens::{tk, TokenKind};
use cool_span::Span;
use paste::paste;

macro_rules! define_expr {
    { $($variant:ident,)+ } => {
        paste! {
            #[derive(Clone)]
            pub enum Expr {
                $($variant([<$variant Expr>]),)+
            }
        }

        impl ParseTree for Expr {
            fn span(&self) -> Span {
                match self {
                    $(Self::$variant(e) => e.span(),)+
                }
            }
        }

        impl std::fmt::Debug for Expr {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$variant(e) => std::fmt::Debug::fmt(e, f),)+
                }
            }
        }

        paste! {
            $(
                impl From<[<$variant Expr>]> for Expr {
                    fn from(expr: [<$variant Expr>]) -> Self {
                        Self::$variant(expr)
                    }
                }
            )+
        }
    };
}

define_expr! {
    Access,
    Array,
    Binary,
    Block,
    Deref,
    Fn,
    FnCall,
    Ident,
    Literal,
    Paren,
    Return,
    Subscript,
    Tuple,
}

#[derive(Clone, Debug)]
enum ExprAtom {
    BinOp(BinOp),
    Expr(Expr),
}

impl ExprAtom {
    #[inline]
    pub fn into_expr(self) -> Expr {
        match self {
            Self::Expr(expr) => expr,
            _ => panic!("not an expression"),
        }
    }
}

impl From<BinOp> for ExprAtom {
    #[inline]
    fn from(bin_op: BinOp) -> Self {
        Self::BinOp(bin_op)
    }
}

impl From<Expr> for ExprAtom {
    #[inline]
    fn from(expr: Expr) -> Self {
        Self::Expr(expr)
    }
}

impl Parser<'_> {
    pub fn parse_expr(&mut self) -> ParseResult<Expr> {
        let expr = self.parse_primary_expr()?;

        let (first_bin_op, second_expr) = match BinOp::from_token_kind(self.peek().kind) {
            Some(bin_op) => {
                self.bump();
                (bin_op, self.parse_primary_expr()?)
            }
            None => return Ok(expr),
        };

        let mut atoms = vec![ExprAtom::Expr(expr), ExprAtom::Expr(second_expr)];
        let mut bin_ops = vec![first_bin_op];

        while let Some(bin_op) = BinOp::from_token_kind(self.peek().kind) {
            self.bump();

            while let Some(&last_bin_op) = bin_ops.last() {
                if last_bin_op.precedence() < bin_op.precedence() {
                    break;
                }

                atoms.push(last_bin_op.into());
                bin_ops.pop();
            }

            bin_ops.push(bin_op);

            let next_expr = self.parse_primary_expr()?;
            atoms.push(next_expr.into());
        }

        while let Some(bin_op) = bin_ops.pop() {
            atoms.push(bin_op.into());
        }

        let mut atom_stack = Vec::<ExprAtom>::new();

        for atom in atoms.drain(..) {
            match atom {
                ExprAtom::Expr(_) => {
                    atom_stack.push(atom);
                }
                ExprAtom::BinOp(bin_op) => {
                    let rhs = atom_stack.pop().unwrap().into_expr();
                    let lhs = atom_stack.pop().unwrap().into_expr();

                    atom_stack.push(ExprAtom::Expr(
                        BinaryExpr {
                            bin_op,
                            lhs: lhs.into(),
                            rhs: rhs.into(),
                        }
                        .into(),
                    ));
                }
            }
        }

        debug_assert!(!atom_stack.is_empty());
        Ok(atom_stack.pop().unwrap().into_expr())
    }

    fn parse_primary_expr(&mut self) -> ParseResult<Expr> {
        let mut expr: Expr = match self.peek().kind {
            tk::OPEN_BRACKET => self.parse_array_expr()?.into(),
            tk::OPEN_BRACE => self.parse_block_expr()?.into(),
            tk::OPEN_PAREN => self.parse_paren_or_tuple_expr()?.into(),
            tk::KW_RETURN => self.parse_return_expr()?.into(),
            TokenKind::Ident(_) => self.parse_ident_expr()?.into(),
            TokenKind::Prefix(_) | TokenKind::Literal(_) => self.parse_literal_expr()?.into(),
            _ => {
                return self.peek_error(&[
                    tk::OPEN_BRACKET,
                    tk::OPEN_BRACE,
                    tk::OPEN_PAREN,
                    tk::KW_RETURN,
                    tk::ANY_IDENT,
                    tk::ANY_LITERAL,
                ])
            }
        };

        loop {
            expr = match &expr {
                Expr::Access(_)
                | Expr::Block(_)
                | Expr::Deref(_)
                | Expr::FnCall(_)
                | Expr::Ident(_)
                | Expr::Paren(_)
                | Expr::Subscript(_) => match self.peek().kind {
                    tk::DOT => self.continue_parse_access_or_deref_expr(Box::new(expr))?,
                    tk::OPEN_PAREN => self.continue_parse_fn_call_expr(Box::new(expr))?.into(),
                    tk::OPEN_BRACKET => self.continue_parse_subscript_expr(Box::new(expr))?.into(),
                    _ => break,
                },
                _ => break,
            }
        }

        Ok(expr)
    }

    fn continue_parse_access_or_deref_expr(&mut self, base: Box<Expr>) -> ParseResult<Expr> {
        self.bump_expect(&tk::DOT)?;
        let next_token = self.bump();

        let expr: Expr = match next_token.kind {
            TokenKind::Ident(symbol) => AccessExpr {
                base,
                ident: Ident {
                    span: next_token.span,
                    symbol,
                },
            }
            .into(),
            tk::STAR => DerefExpr {
                span: base.span().to(next_token.span),
                base,
            }
            .into(),
            _ => self.error(next_token, &[tk::ANY_IDENT, tk::STAR])?,
        };

        Ok(expr)
    }

    fn parse_paren_or_tuple_expr(&mut self) -> ParseResult<Expr> {
        let start_token = self.bump_expect(&tk::OPEN_PAREN)?;
        let mut exprs = Vec::<Expr>::new();

        let (end_token, has_trailing_comma) = match self.peek().kind {
            tk::CLOSE_PAREN => (self.bump(), false),
            _ => loop {
                exprs.push(self.parse_expr()?);

                if self.bump_if_eq(tk::COMMA).is_some() {
                    if let Some(end_token) = self.bump_if_eq(tk::CLOSE_PAREN) {
                        break (end_token, true);
                    }
                } else if let Some(end_token) = self.bump_if_eq(tk::CLOSE_PAREN) {
                    break (end_token, false);
                } else {
                    return self.peek_error(&[tk::COMMA, tk::CLOSE_PAREN]);
                }
            },
        };

        let span = start_token.span.to(end_token.span);
        let expr: Expr = if exprs.len() == 1 && !has_trailing_comma {
            ParenExpr {
                span,
                inner: Box::new(exprs.remove(0)),
            }
            .into()
        } else {
            TupleExpr {
                span,
                elems: exprs,
                has_trailing_comma,
            }
            .into()
        };

        Ok(expr)
    }
}
