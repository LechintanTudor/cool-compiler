mod array_ty;
mod fn_ty;
mod many_ptr_ty;
mod module_ty;
mod path_ty;
mod ptr_ty;
mod slice_ty;
mod tuple_ty;

pub use self::array_ty::*;
pub use self::fn_ty::*;
pub use self::many_ptr_ty::*;
pub use self::module_ty::*;
pub use self::path_ty::*;
pub use self::ptr_ty::*;
pub use self::slice_ty::*;
pub use self::tuple_ty::*;
use crate::{ParseResult, Parser};
use cool_lexer::tokens::{tk, TokenKind};
use cool_span::{Section, Span};
use derive_more::From;

#[derive(Clone, From, Debug)]
pub enum Ty {
    Array(ArrayTy),
    Fn(FnTy),
    ManyPtr(ManyPtrTy),
    Module(ModuleTy),
    Path(PathTy),
    Ptr(PtrTy),
    Slice(SliceTy),
    Tuple(TupleTy),
}

impl Section for Ty {
    fn span(&self) -> Span {
        match self {
            Ty::Array(ty) => ty.span(),
            Ty::Fn(ty) => ty.span(),
            Ty::ManyPtr(ty) => ty.span(),
            Ty::Module(ty) => ty.span(),
            Ty::Path(ty) => ty.span(),
            Ty::Ptr(ty) => ty.span(),
            Ty::Slice(ty) => ty.span(),
            Ty::Tuple(ty) => ty.span(),
        }
    }
}

impl Parser<'_> {
    pub fn parse_ty(&mut self) -> ParseResult<Ty> {
        let ty: Ty = match self.peek().kind {
            tk::KW_EXTERN | tk::KW_FN => self.parse_fn_ty()?.into(),
            tk::KW_MODULE => self.parse_module_ty()?.into(),
            TokenKind::Ident(_) => self.parse_path_ty()?.into(),
            tk::STAR => self.parse_ptr_ty()?.into(),
            tk::OPEN_PAREN => self.parse_tuple_ty()?.into(),
            tk::OPEN_BRACKET => self.parse_array_or_slice_ty()?,
            _ => {
                return self.peek_error(&[
                    tk::KW_MODULE,
                    tk::KW_EXTERN,
                    tk::KW_FN,
                    tk::DIAG_IDENT,
                    tk::STAR,
                    tk::OPEN_PAREN,
                    tk::OPEN_BRACKET,
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
