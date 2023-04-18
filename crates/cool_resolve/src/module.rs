use crate::{ItemId, ItemPathBuf};
use cool_lexer::symbols::Symbol;
use rustc_hash::FxHashMap;

#[derive(Clone, Debug)]
pub struct Module {
    pub path: ItemPathBuf,
    pub elems: FxHashMap<Symbol, ModuleElem>,
}

impl Module {
    pub fn from_path<P>(path: P) -> Self
    where
        P: Into<ItemPathBuf>,
    {
        Self {
            path: path.into(),
            elems: Default::default(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ModuleElem {
    pub is_exported: bool,
    pub item_id: ItemId,
}
