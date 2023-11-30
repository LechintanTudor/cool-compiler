mod array_ty;
mod fn_ty;
mod item_ty;
mod ptr_ty;
mod tuple_ty;
mod variant_ty;

pub use self::array_ty::*;
pub use self::fn_ty::*;
pub use self::item_ty::*;
pub use self::ptr_ty::*;
pub use self::tuple_ty::*;
pub use self::variant_ty::*;

use crate::{IdentPath, ParseResult, Parser};
use cool_collections::define_index_newtype;
use cool_derive::Section;
use cool_lexer::{tk, TokenKind};
use derive_more::From;

define_index_newtype!(TyId);

#[derive(Clone, Section, From, Debug)]
pub enum Ty {
    Array(ArrayTy),
    Fn(FnTy),
    Item(ItemTy),
    ManyPtr(ManyPtrTy),
    Paren(ParenTy),
    Path(IdentPath),
    Ptr(PtrTy),
    Slice(SliceTy),
    Tuple(TupleTy),
    Variant(VariantTy),
}

impl Parser<'_> {
    pub fn parse_ty(&mut self) -> ParseResult<TyId> {
        let ty = self.parse_non_variant_ty()?;

        if self.peek().kind == tk::or {
            self.continue_parse_variant_ty(ty)
        } else {
            Ok(ty)
        }
    }

    pub fn parse_non_variant_ty(&mut self) -> ParseResult<TyId> {
        let peeked_token = self.peek();

        match peeked_token.kind {
            TokenKind::Ident(_) => self.parse_path_ty(),
            tk::kw_alias | tk::kw_module => self.parse_item_ty(),
            tk::kw_extern | tk::kw_fn => self.parse_fn_ty(),
            tk::star => self.parse_ptr_ty(),
            tk::open_paren => self.parse_paren_or_tuple_ty(),
            tk::open_bracket => self.parse_array_or_slice_or_many_ptr_ty(),
            _ => todo!(),
        }
    }

    fn parse_path_ty(&mut self) -> ParseResult<TyId> {
        let path = self.parse_ident_path()?;
        Ok(self.add_ty(path))
    }
}
