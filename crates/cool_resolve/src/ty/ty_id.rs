use crate::{resolve_fields_size_align, AnyTy, Field, ResolveTy, StructTyDef, ValueTy};
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug)]
pub struct TyId(&'static ResolveTy);

impl TyId {
    #[inline]
    pub(crate) fn new(resolve_ty: &'static ResolveTy) -> Self {
        Self(resolve_ty)
    }

    #[inline]
    pub fn is_inferred(&self) -> bool {
        matches!(self.0.ty, AnyTy::Infer(_))
    }

    pub fn get_size(&self) -> u64 {
        match self.0.ty {
            AnyTy::Value(ValueTy::Struct(struct_ty)) => struct_ty.def.lock().unwrap().unwrap().size,
            _ => self.0.size,
        }
    }

    pub fn get_align(&self) -> u64 {
        match self.0.ty {
            AnyTy::Value(ValueTy::Struct(struct_ty)) => {
                struct_ty.def.lock().unwrap().unwrap().align
            }
            _ => self.0.align,
        }
    }

    pub fn define_struct(&self, mut fields: Vec<Field>) {
        let AnyTy::Value(ValueTy::Struct(struct_ty)) = self.0.ty else {
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
        std::ptr::eq(self, other)
    }
}

impl Eq for TyId {}

impl Hash for TyId {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        std::ptr::hash(self, state);
    }
}
