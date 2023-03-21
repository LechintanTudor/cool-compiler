mod symbol_resolver;
mod symbol_adder;

pub use self::symbol_resolver::*;
pub use self::symbol_adder::*;
use crate::ItemPathBuf;
use cool_arena::SliceArena;
use cool_collections::{id_newtype, IdIndexedVec, SmallVecMap};
use cool_lexer::symbols::Symbol;
use rustc_hash::FxHashMap;

id_newtype!(ItemId);
id_newtype!(ModuleId);
id_newtype!(FrameId);
id_newtype!(BindingId);

impl ModuleId {
    #[inline]
    pub fn for_builtins() -> Self {
        Self::new_unwrap(1)
    }

    #[inline]
    pub const fn as_item_id(&self) -> ItemId {
        ItemId(self.0)
    }
}

impl From<ModuleId> for ItemId {
    #[inline]
    fn from(module_id: ModuleId) -> Self {
        Self(module_id.0) 
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

#[derive(Clone, Copy, Debug)]
pub struct ModuleElem {
    pub is_exported: bool,
    pub kind: SymbolKind,
}

#[derive(Clone, Debug)]
pub struct Frame {
    pub parent_id: ScopeId,
    pub bindings: SmallVecMap<Symbol, BindingId, 2>,
}

#[derive(Clone, Copy, Debug)]
pub struct Binding {
    pub is_mutable: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum SymbolKind {
    Item(ItemId),
    Binding(BindingId),
}

impl SymbolKind {
    #[inline]
    pub fn as_item_id(&self) -> Option<ItemId> {
        match self {
            Self::Item(item_id) => Some(*item_id),
            _ => None,
        }
    }

    pub fn as_binding_id(&self) -> Option<BindingId> {
        match self {
            Self::Binding(binding_id) => Some(*binding_id),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct ResolveTable {
    items: SliceArena<ItemId, Symbol>,
    modules: FxHashMap<ModuleId, Module>,
    frames: IdIndexedVec<FrameId, Frame>,
    bindings: IdIndexedVec<BindingId, Binding>,
}
