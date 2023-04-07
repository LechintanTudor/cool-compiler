use crate::{BindingId, ItemPathBuf};
use cool_collections::{id_newtype, SmallVecMap};
use cool_lexer::symbols::Symbol;
use rustc_hash::FxHashMap;

id_newtype!(ItemId);
id_newtype!(ModuleId);
id_newtype!(FrameId);

impl ModuleId {
    #[inline]
    pub fn for_builtins() -> Self {
        Self::new_unwrap(1)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ScopeId {
    Frame(FrameId),
    Module(ModuleId),
}

impl From<FrameId> for ScopeId {
    #[inline]
    fn from(frame_id: FrameId) -> Self {
        Self::Frame(frame_id)
    }
}

impl From<ModuleId> for ScopeId {
    #[inline]
    fn from(module_id: ModuleId) -> Self {
        Self::Module(module_id)
    }
}

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

#[derive(Clone, Debug)]
pub struct Frame {
    pub parent_id: ScopeId,
    pub bindings: SmallVecMap<Symbol, BindingId, 2>,
}

impl Frame {
    #[inline]
    pub fn new(parent_id: ScopeId) -> Self {
        Self {
            parent_id,
            bindings: Default::default(),
        }
    }
}
