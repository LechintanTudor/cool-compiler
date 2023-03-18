mod array_expr;
mod block_expr;
mod fn_call_expr;
mod ident_expr;
mod literal_expr;
mod paren_expr;
mod path_expr;
mod return_expr;
mod tuple_expr;

pub use self::array_expr::*;
pub use self::block_expr::*;
pub use self::fn_call_expr::*;
pub use self::ident_expr::*;
pub use self::literal_expr::*;
pub use self::paren_expr::*;
pub use self::path_expr::*;
pub use self::return_expr::*;
pub use self::tuple_expr::*;
use crate::{ParseResult, ParseTree, Parser};
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
    Array,
    Block,
    FnCall,
    Ident,
    Literal,
    Paren,
    Path,
    Return,
    Tuple,
}

impl Parser<'_> {
    pub fn parse_expr(&mut self) -> ParseResult<Expr> {
        let expr: Expr = match self.peek().kind {
            tk::OPEN_BRACKET => self.parse_array_expr()?.into(),
            tk::OPEN_BRACE => self.parse_block_expr()?.into(),
            tk::OPEN_PAREN => self.parse_paren_or_tuple_expr()?.into(),
            tk::KW_RETURN => self.parse_return_expr()?.into(),
            TokenKind::Ident(_) => self.parse_ident_or_path_or_fn_call_expr()?.into(),
            TokenKind::Prefix(_) | TokenKind::Literal(_) => self.parse_literal_expr()?.into(),
            _ => {
                return self.peek_error(&[
                    tk::OPEN_BRACKET,
                    tk::OPEN_BRACE,
                    tk::OPEN_PAREN,
                    tk::ANY_IDENT,
                    tk::ANY_LITERAL,
                ])
            }
        };

        Ok(expr)
    }

    fn parse_paren_or_tuple_expr(&mut self) -> ParseResult<Expr> {
        let start_token = self.bump_expect(&tk::OPEN_PAREN)?;

        let mut exprs = Vec::<Expr>::new();
        let (end_token, has_trailing_comma) = match self.peek().kind {
            tk::CLOSE_BRACKET => (self.bump(), false),
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
                exprs,
                has_trailing_comma,
            }
            .into()
        };

        Ok(expr)
    }

    fn parse_ident_or_path_or_fn_call_expr(&mut self) -> ParseResult<Expr> {
        let ident_expr: Expr = self.parse_ident_expr()?.into();

        let expr: Expr = match self.peek().kind {
            tk::DOT => {
                let path_expr: Expr = self.continue_parse_path_expr(Box::new(ident_expr))?.into();

                if self.peek().kind == tk::OPEN_PAREN {
                    self.continue_parse_fn_call_expr(Box::new(path_expr))?
                        .into()
                } else {
                    path_expr
                }
            }
            tk::OPEN_PAREN => self
                .continue_parse_fn_call_expr(Box::new(ident_expr))?
                .into(),
            _ => ident_expr,
        };

        Ok(expr)
    }
}
