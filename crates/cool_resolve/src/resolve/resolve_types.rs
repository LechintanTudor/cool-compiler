use crate::ty::TyId;
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

#[derive(Clone, Copy, Debug)]
pub enum ItemKind {
    Module(ModuleId),
    Ty(TyId),
    Binding(BindingId),
}

impl From<ModuleId> for ItemKind {
    #[inline]
    fn from(module_id: ModuleId) -> Self {
        Self::Module(module_id)
    }
}

impl From<TyId> for ItemKind {
    #[inline]
    fn from(ty_id: TyId) -> Self {
        Self::Ty(ty_id)
    }
}

impl From<BindingId> for ItemKind {
    #[inline]
    fn from(binding_id: BindingId) -> Self {
        Self::Binding(binding_id)
    }
}

impl ItemKind {
    #[inline]
    pub fn as_module_id(&self) -> Option<ModuleId> {
        match self {
            Self::Module(module_id) => Some(*module_id),
            _ => None,
        }
    }

    #[inline]
    pub fn as_ty_id(&self) -> Option<TyId> {
        match self {
            Self::Ty(ty_id) => Some(*ty_id),
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Mutability {
    Const,
    Immutable,
    Mutable,
}

impl Mutability {
    #[inline]
    pub fn local(is_mutable: bool) -> Self {
        if is_mutable {
            Self::Mutable
        } else {
            Self::Immutable
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ExprKind {
    Lvalue,
    Rvalue,
}

#[derive(Clone, Copy, Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub ty_id: TyId,
}

#[derive(Clone, Copy, Debug)]
pub struct Binding {
    pub mutability: Mutability,
    pub ty_id: TyId,
}
