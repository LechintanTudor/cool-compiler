use crate::{BindingId, ConstId, ModuleId, TyId};
use derive_more::{From, TryInto};

#[derive(Clone, Copy, PartialEq, Eq, Hash, From, TryInto, Debug)]
pub enum ItemKind {
    Binding(BindingId),
    Const(ConstId),
    Module(ModuleId),
    Ty(TyId),
}

impl ItemKind {
    #[inline]
    #[must_use]
    pub fn try_as_const(&self) -> Option<ConstId> {
        match self {
            Self::Const(const_id) => Some(*const_id),
            _ => None,
        }
    }

    #[inline]
    #[must_use]
    pub fn try_as_ty(&self) -> Option<TyId> {
        match self {
            Self::Ty(ty_id) => Some(*ty_id),
            _ => None,
        }
    }
}
