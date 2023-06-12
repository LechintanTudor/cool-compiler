use crate::{Field, TyId};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct PtrTy {
    pub pointee: TyId,
    pub is_mutable: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ManyPtrTy {
    pub pointee: TyId,
    pub is_mutable: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SliceTy {
    pub fields: [Field; 2],
}

impl SliceTy {
    #[inline]
    pub fn ptr_field(&self) -> &Field {
        &self.fields[0]
    }

    #[inline]
    pub fn len_field(&self) -> &Field {
        &self.fields[1]
    }
}
