mod fn_ty;
mod module_ty;
mod path_ty;
mod pointer_ty;
mod tuple_ty;

pub use self::fn_ty::*;
pub use self::module_ty::*;
pub use self::path_ty::*;
pub use self::pointer_ty::*;
pub use self::tuple_ty::*;
use crate::{ParseResult, Parser};
use cool_lexer::tokens::{tk, TokenKind};
use cool_span::{Section, Span};
use derive_more::From;
use paste::paste;

macro_rules! define_ty {
    { $($Variant:ident,)+ } => {
        paste! {
            #[derive(Clone, From, Debug)]
            pub enum Ty {
                $($Variant([<$Variant Ty>]),)+
            }
        }

        impl Section for Ty {
            fn span(&self) -> Span {
                match self {
                    $(Self::$Variant(i) => i.span(),)+
                }
            }
        }
    };
}

define_ty! {
    Fn,
    Module,
    Path,
    Pointer,
    Tuple,
}

impl Parser<'_> {
    pub fn parse_ty(&mut self) -> ParseResult<Ty> {
        let ty: Ty = match self.peek().kind {
            tk::KW_EXTERN | tk::KW_FN => self.parse_fn_ty()?.into(),
            tk::KW_MODULE => self.parse_module_ty()?.into(),
            TokenKind::Ident(_) => self.parse_path_ty()?.into(),
            tk::STAR => self.parse_pointer_ty()?.into(),
            tk::OPEN_PAREN => self.parse_tuple_ty()?.into(),
            _ => {
                return self.peek_error(&[
                    tk::KW_MODULE,
                    tk::KW_EXTERN,
                    tk::KW_FN,
                    tk::ANY_IDENT,
                    tk::STAR,
                    tk::OPEN_PAREN,
                ]);
            }
        };

        Ok(ty)
    }
}
