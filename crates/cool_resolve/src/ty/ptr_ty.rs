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
    pub const PTR_FIELD_INDEX: u32 = 0;
    pub const LEN_FIELD_INDEX: u32 = 1;

    #[inline]
    pub fn ptr_field(&self) -> &Field {
        &self.fields[Self::PTR_FIELD_INDEX as usize]
    }

    #[inline]
    pub fn len_field(&self) -> &Field {
        &self.fields[Self::LEN_FIELD_INDEX as usize]
    }

    #[inline]
    pub fn ptr_ty(&self) -> &ManyPtrTy {
        self.ptr_field().ty_id.as_many_ptr().unwrap()
    }
}
