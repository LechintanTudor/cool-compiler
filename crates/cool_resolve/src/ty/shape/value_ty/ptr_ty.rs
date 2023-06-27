use crate::TyId;
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
    pub elem: TyId,
    pub is_mutable: bool,
}

impl SliceTy {
    pub const PTR_FIELD_INDEX: u32 = 0;
    pub const LEN_FIELD_INDEX: u32 = 1;
}

impl fmt::Display for SliceTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut_display_str = if self.is_mutable { "mut " } else { "" };
        write!(f, "[]{}{}", mut_display_str, self.elem)
    }
}