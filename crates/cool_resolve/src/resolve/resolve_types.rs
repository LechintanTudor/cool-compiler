use crate::ItemPathBuf;
use cool_collections::{id_newtype, SmallVecMap};
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
    pub kind: SymbolKind,
}

#[derive(Clone, Debug)]
pub struct Frame {
    pub parent_id: ScopeId,
    pub bindings: SmallVecMap<Symbol, BindingId, 2>,
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Binding {
    pub is_mutable: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum SymbolKind {
    Item(ItemId),
    Binding(BindingId),
}

impl From<ItemId> for SymbolKind {
    #[inline]
    fn from(item_id: ItemId) -> Self {
        Self::Item(item_id)
    }
}

impl From<BindingId> for SymbolKind {
    #[inline]
    fn from(binding_id: BindingId) -> Self {
        Self::Binding(binding_id)
    }
}

impl SymbolKind {
    #[inline]
    pub fn as_item_id(&self) -> Option<ItemId> {
        match self {
            Self::Item(item_id) => Some(*item_id),
            _ => None,
        }
    }

    #[inline]
    pub fn as_binding_id(&self) -> Option<BindingId> {
        match self {
            Self::Binding(binding_id) => Some(*binding_id),
            _ => None,
        }
    }
}
