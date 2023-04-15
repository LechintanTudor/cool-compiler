use crate::{BindingId, ItemId, ItemPathBuf, ResolveTable};
use cool_collections::{id_newtype, SmallVecMap};
use cool_lexer::symbols::Symbol;
use rustc_hash::FxHashMap;
use std::ops;

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

impl ops::Index<ModuleId> for ResolveTable {
    type Output = Module;

    #[inline]
    fn index(&self, module_id: ModuleId) -> &Self::Output {
        &self.modules[module_id]
    }
}
