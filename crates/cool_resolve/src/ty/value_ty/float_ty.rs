use crate::{PrimitiveTys, ResolveTy, ValueTy};
use derive_more::Display;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Display, Debug)]
pub enum FloatTy {
    #[display(fmt = "f32")]
    F32,
    #[display(fmt = "f64")]
    F64,
}

impl FloatTy {
    pub fn to_resolve_ty(&self, primitives: &PrimitiveTys) -> ResolveTy {
        let mk_resolve_ty = |size, align| {
            ResolveTy {
                size,
                align,
                ty: ValueTy::Float(*self),
            }
        };

        match self {
            FloatTy::F32 => mk_resolve_ty(4, primitives.f32_align),
            FloatTy::F64 => mk_resolve_ty(8, primitives.f64_align),
        }
    }
}
