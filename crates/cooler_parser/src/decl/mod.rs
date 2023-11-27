mod import;

pub use self::import::*;

use crate::ItemId;
use cool_collections::define_index_newtype;
use cool_derive::Section;
use cool_span::Span;

define_index_newtype!(DeclId);

#[derive(Clone, Section, Debug)]
pub struct Decl {
    pub span: Span,
    pub is_exported: bool,
    pub kind: DeclKind,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum DeclKind {
    Item(ItemId),
    Import(ImportId),
}
