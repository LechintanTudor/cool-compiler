mod item_ty;
mod tuple_ty;

pub use self::item_ty::*;
pub use self::tuple_ty::*;

use crate::{IdentPath, ParseResult, Parser};
use cool_collections::define_index_newtype;
use cool_derive::Section;
use cool_lexer::{tk, TokenKind};
use derive_more::From;

define_index_newtype!(TyId);

#[derive(Clone, Section, From, Debug)]
pub enum Ty {
    Item(ItemTy),
    Paren(ParenTy),
    Path(IdentPath),
    Tuple(TupleTy),
}

impl Parser<'_> {
    pub fn parse_ty(&mut self) -> ParseResult<TyId> {
        let peeked_token = self.peek();

        match peeked_token.kind {
            TokenKind::Ident(_) => {
                let path_ty = Ty::Path(self.parse_ident_path()?);
                Ok(self.data.tys.push(path_ty))
            }
            tk::kw_alias | tk::kw_module => self.parse_item_ty(),
            tk::open_paren => self.parse_paren_or_tuple_ty(),
            _ => todo!(),
        }
    }
}
