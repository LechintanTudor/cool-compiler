use crate::{ParseResult, Parser};
use cool_collections::define_index_newtype;
use cool_derive::Section;

define_index_newtype!(TyId);

#[derive(Clone, Section, Debug)]
pub enum Ty {
    // Empty
}

impl Parser<'_> {
    pub fn parse_ty(&mut self) -> ParseResult<TyId> {
        todo!()
    }
}
