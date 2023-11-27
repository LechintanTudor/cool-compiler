use crate::DeclId;
use cool_collections::{define_index_newtype, SmallVec};
use cool_derive::Section;
use cool_span::Span;

define_index_newtype!(ModuleId);

#[derive(Clone, Section, Debug)]
pub struct Module {
    pub span: Span,
    pub kind: ModuleKind,
    pub decls: SmallVec<DeclId, 4>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ModuleKind {
    File,
    Inline,
}
