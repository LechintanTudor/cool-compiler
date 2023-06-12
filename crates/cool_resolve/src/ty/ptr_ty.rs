use crate::{Field, TyId};
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct PtrTy {
    pub pointee: TyId,
    pub is_mutable: bool,
}

impl fmt::Display for PtrTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut_display_str = if self.is_mutable { "mut " } else { "" };
        write!(f, "*{}{}", mut_display_str, self.pointee)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ManyPtrTy {
    pub pointee: TyId,
    pub is_mutable: bool,
}

impl fmt::Display for ManyPtrTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut_display_str = if self.is_mutable { "mut " } else { "" };
        write!(f, "[*]{}{}", mut_display_str, self.pointee)
    }
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

impl fmt::Display for SliceTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ptr_ty = self.ptr_ty();
        let mut_display_str = if ptr_ty.is_mutable { "mut " } else { "" };
        write!(f, "[]{}{}", mut_display_str, ptr_ty.pointee)
    }
}
