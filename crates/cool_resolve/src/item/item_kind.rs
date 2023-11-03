use crate::{BindingId, ModuleId, TyId};
use derive_more::{From, TryInto};

#[derive(Clone, Copy, PartialEq, Eq, Hash, From, TryInto, Debug)]
pub enum ItemKind {
    Module(ModuleId),
    Ty(TyId),
    Binding(BindingId),
}

impl ItemKind {
    #[inline]
    #[must_use]
    pub fn try_as_ty(&self) -> Option<TyId> {
        match self {
            Self::Ty(ty_id) => Some(*ty_id),
            _ => None,
        }
    }
}
