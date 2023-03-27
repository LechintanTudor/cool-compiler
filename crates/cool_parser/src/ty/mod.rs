mod fn_ty;
mod path_ty;
mod pointer_ty;
mod tuple_ty;

pub use self::fn_ty::*;
pub use self::path_ty::*;
pub use self::pointer_ty::*;
pub use self::tuple_ty::*;
use crate::{ParseResult, ParseTree, Parser};
use cool_lexer::tokens::{tk, TokenKind};
use cool_span::Span;
use paste::paste;

macro_rules! define_ty {
    { $($variant:ident,)+ } => {
        paste! {
            #[derive(Clone)]
            pub enum Ty {
                $($variant([<$variant Ty>]),)+
            }
        }

        impl ParseTree for Ty {
            fn span(&self) -> Span {
                match self {
                    $(Self::$variant(i) => i.span(),)+
                }
            }
        }

        impl std::fmt::Debug for Ty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$variant(i) => std::fmt::Debug::fmt(i, f),)+
                }
            }
        }

        paste! {
            $(
                impl From<[<$variant Ty>]> for Ty {
                    fn from(item: [<$variant Ty>]) -> Self {
                        Self::$variant(item)
                    }
                }
            )+
        }
    };
}

define_ty! {
    Fn,
    Path,
    Pointer,
    Tuple,
}

impl Parser<'_> {
    pub fn parse_ty(&mut self) -> ParseResult<Ty> {
        let ty: Ty = match self.peek().kind {
            tk::OPEN_PAREN => self.parse_tuple_ty()?.into(),
            tk::KW_FN | tk::KW_EXTERN => self.parse_fn_ty()?.into(),
            tk::STAR => self.parse_pointer_ty()?.into(),
            TokenKind::Ident(_) => self.parse_path_ty()?.into(),
            _ => {
                return self.peek_error(&[
                    tk::OPEN_PAREN,
                    tk::KW_FN,
                    tk::KW_EXTERN,
                    tk::STAR,
                    tk::ANY_IDENT,
                ]);
            }
        };

        Ok(ty)
    }
}
