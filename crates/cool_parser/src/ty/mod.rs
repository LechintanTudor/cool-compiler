mod array_ty;
mod fn_ty;
mod item_ty;
mod many_ptr_ty;
mod paren_ty;
mod path_ty;
mod ptr_ty;
mod slice_ty;

pub use self::array_ty::*;
pub use self::fn_ty::*;
pub use self::item_ty::*;
pub use self::many_ptr_ty::*;
pub use self::paren_ty::*;
pub use self::path_ty::*;
pub use self::ptr_ty::*;
pub use self::slice_ty::*;
use crate::{ParseResult, Parser};
use cool_lexer::{tk, TokenKind};
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

            impl Section for Ty {
                fn span(&self) -> Span {
                    match self {
                        $(Self::$Variant(ty) => ty.span(),)+
                    }
                }
            }
        }
    };
}

define_ty! {
    Array,
    Fn,
    Item,
    ManyPtr,
    Paren,
    Path,
    Ptr,
    Slice,
    Tuple,
    Variant,
}

impl Parser<'_> {
    pub fn parse_ty(&mut self) -> ParseResult<Ty> {
        let ty: Ty = match self.peek().kind {
            tk::KW_CRATE | tk::KW_SUPER | tk::KW_SELF | TokenKind::Ident(_) => {
                self.parse_path_ty()?.into()
            }
            tk::KW_EXTERN | tk::KW_FN => self.parse_fn_ty()?.into(),
            tk::KW_MODULE | tk::KW_TYPE => self.parse_item_ty()?.into(),
            tk::OPEN_BRACKET => self.parse_array_or_slice_ty()?,
            tk::OPEN_PAREN => self.parse_paren_ty()?,
            tk::STAR => self.parse_ptr_ty()?.into(),
            _ => {
                return self.peek_error(&[
                    tk::DIAG_IDENT,
                    tk::KW_CRATE,
                    tk::KW_EXTERN,
                    tk::KW_FN,
                    tk::KW_MODULE,
                    tk::KW_SELF,
                    tk::KW_SUPER,
                    tk::KW_TYPE,
                    tk::OPEN_BRACKET,
                    tk::OPEN_PAREN,
                    tk::STAR,
                ]);
            }
        };

        Ok(ty)
    }

    pub fn parse_array_or_slice_ty(&mut self) -> ParseResult<Ty> {
        let open_bracket = self.bump_expect(&tk::OPEN_BRACKET)?;

        let ty = match self.peek().kind {
            tk::CLOSE_BRACKET => self.continue_parse_slice_ty(open_bracket)?.into(),
            tk::STAR => self.continue_parse_many_ptr_ty(open_bracket)?.into(),
            _ => self.continue_parse_array_ty(open_bracket)?.into(),
        };

        Ok(ty)
    }
}
