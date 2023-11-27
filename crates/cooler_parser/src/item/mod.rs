mod module_item;
mod struct_item;

pub use self::module_item::*;
pub use self::struct_item::*;

use crate::{ExprId, Ident, TyId};
use cool_collections::define_index_newtype;
use cool_derive::Section;
use cool_span::Span;
use derive_more::From;

define_index_newtype!(ItemId);

#[derive(Clone, Debug)]
pub enum Item {
    File(ModuleId),
    Inline(InlineItem),
}

impl Item {
    #[inline]
    #[must_use]
    pub fn kind(&self) -> ItemKind {
        match self {
            Self::File(module_id) => ItemKind::Module(*module_id),
            Self::Inline(item) => item.kind,
        }
    }
}

#[derive(Clone, Section, Debug)]
pub struct InlineItem {
    pub span: Span,
    pub is_exported: bool,
    pub ident: Ident,
    pub ty: Option<TyId>,
    pub kind: ItemKind,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, From, Debug)]
pub enum ItemKind {
    Alias(TyId),
    Expr(ExprId),
    Module(ModuleId),
    Struct(StructId),
}
