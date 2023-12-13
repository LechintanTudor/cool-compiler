use crate::{CrateId, Item};
use cool_collections::ahash::AHashMap;
use cool_collections::{define_index_newtype, SmallVec};
use cool_lexer::Symbol;

define_index_newtype!(ModuleId);

#[derive(Debug)]
pub struct Module {
    pub crate_id: CrateId,
    pub path: SmallVec<Symbol, 4>,
    pub items: AHashMap<Symbol, ModuleItem>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ModuleItem {
    pub is_exported: bool,
    pub item: Item,
}
