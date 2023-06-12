use crate::{resolve_fields_size_align, AnyTy, Field, ResolveTy, StructTyDef, ValueTy};
use std::hash::{Hash, Hasher};
use std::{fmt, ops};

#[derive(Clone, Copy, Debug)]
pub struct TyId(&'static ResolveTy);

impl fmt::Display for TyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl TyId {
    #[inline]
    pub(crate) fn new(resolve_ty: &'static ResolveTy) -> Self {
        Self(resolve_ty)
    }

    pub fn get_size(&self) -> u64 {
        match &self.0.ty {
            AnyTy::Value(ValueTy::Struct(struct_ty)) => {
                struct_ty.def.lock().unwrap().as_ref().unwrap().size
            }
            _ => self.0.size,
        }
    }

    pub fn get_align(&self) -> u64 {
        match &self.0.ty {
            AnyTy::Value(ValueTy::Struct(struct_ty)) => {
                struct_ty.def.lock().unwrap().as_ref().unwrap().align
            }
            _ => self.0.align,
        }
    }

    pub fn is_defined(&self) -> bool {
        match &self.0.ty {
            AnyTy::Value(ValueTy::Struct(struct_ty)) => struct_ty.def.lock().unwrap().is_some(),
            _ => true,
        }
    }

    #[inline]
    pub fn is_zero_sized(&self) -> bool {
        self.get_size() == 0
    }

    pub fn define_struct(&self, mut fields: Vec<Field>) {
        let AnyTy::Value(ValueTy::Struct(struct_ty)) = &self.0.ty else {
            panic!("type is not a struct");
        };

        let mut struct_def = struct_ty.def.lock().unwrap();

        if struct_def.is_some() {
            panic!("struct is already defined");
        }

        let (size, align) = resolve_fields_size_align(&mut fields);

        *struct_def = Some(StructTyDef {
            size,
            align,
            fields: fields.into(),
        });
    }
}

impl PartialEq for TyId {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.0, other.0)
    }
}

impl Eq for TyId {}

impl Hash for TyId {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        std::ptr::hash(self.0, state);
    }
}

impl ops::Deref for TyId {
    type Target = AnyTy;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0.ty
    }
}
