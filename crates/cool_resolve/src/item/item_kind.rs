use crate::{BindingId, ModuleId, TyId};
use derive_more::{From, TryInto};

#[derive(Clone, Copy, PartialEq, Eq, Hash, From, TryInto, Debug)]
pub enum ItemKind {
    Module(ModuleId),
    Ty(TyId),
    Binding(BindingId),
}
