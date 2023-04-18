use crate::TyId;

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
pub struct Binding {
    pub mutability: Mutability,
    pub ty_id: TyId,
}
