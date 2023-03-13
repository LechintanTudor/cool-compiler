mod array_expr;
mod block_expr;
mod fn_call_expr;
mod ident_expr;
mod literal_expr;
mod paren_expr;
mod path_expr;
mod tuple_expr;

pub use self::array_expr::*;
pub use self::block_expr::*;
pub use self::fn_call_expr::*;
pub use self::ident_expr::*;
pub use self::literal_expr::*;
pub use self::paren_expr::*;
pub use self::path_expr::*;
pub use self::tuple_expr::*;
use crate::{ParseResult, ParseTree, Parser, UnexpectedToken};
use cool_lexer::tokens::{tk, Token, TokenKind};
use cool_span::Span;
use paste::paste;

macro_rules! define_expr {
    { $($variant:ident,)+ } => {
        paste! {
            #[derive(Clone, Debug)]
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
    Tuple,
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn parse_expr(&mut self) -> ParseResult<Expr> {
        let expr: Expr = match self.peek().kind {
            tk::OPEN_BRACKET => todo!("array"),
            tk::OPEN_BRACE => self.parse_block_expr()?.into(),
            tk::OPEN_PAREN => todo!("paren or tuple"),
            TokenKind::Ident(_) => self.parse_ident_or_path_or_fn_call_expr()?.into(),
            TokenKind::Literal(_) => self.parse_literal_expr()?.into(),
            _ => Err(UnexpectedToken {
                found: self.peek(),
                expected: &[
                    tk::OPEN_BRACKET,
                    tk::OPEN_BRACE,
                    tk::OPEN_PAREN,
                    tk::ANY_IDENT,
                    tk::ANY_LITERAL,
                ],
            })?,
        };

        Ok(expr)
    }

    pub fn parse_ident_or_path_or_fn_call_expr(&mut self) -> ParseResult<Expr> {
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
