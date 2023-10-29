use crate::{BindingId, ModuleId, TyId};
use derive_more::From;

#[derive(Clone, Copy, PartialEq, Eq, Hash, From, Debug)]
pub enum ItemKind {
    Module(ModuleId),
    Ty(TyId),
    Binding(BindingId),
}
