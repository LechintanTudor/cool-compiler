use crate::ty::TyId;
use crate::{BindingId, ModuleId};

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
