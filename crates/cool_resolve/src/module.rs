use crate::{ItemId, ItemPath, ItemPathBuf};
use cool_lexer::Symbol;
use rustc_hash::FxHashMap;

#[derive(Clone, Debug)]
pub struct Module {
    pub item_id: ItemId,
    pub elems: FxHashMap<Symbol, ModuleElem>,
}

impl Module {
    #[inline]
    pub fn new(item_id: ItemId) -> Self {
        Self {
            item_id,
            elems: Default::default(),
        }
    }

    #[inline]
    pub fn path(&self) -> ItemPath {
        ItemPath::from(&*self.item_id)
    }

    #[inline]
    pub fn child_path(&self, child_symbol: Symbol) -> ItemPathBuf {
        ItemPathBuf::from_base_and_symbol(&self.item_id, child_symbol)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ModuleElem {
    pub is_exported: bool,
    pub item_id: ItemId,
}
