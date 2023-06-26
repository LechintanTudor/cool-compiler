mod aggregate_ty;
mod primitive_ty_data;

pub use self::aggregate_ty::*;
pub use self::primitive_ty_data::*;
use crate::{FloatTy, IntTy, TyShape, ValueTy};

#[derive(Clone, Debug)]
pub enum TyKind {
    Basic,
    Aggregate(AggregateTy),
}

#[derive(Clone, Debug)]
pub struct TyDef {
    pub size: u64,
    pub align: u64,
    pub kind: TyKind,
}

impl TyDef {
    pub fn for_basic(ty_shape: &TyShape, primitives: &PrimitiveTyData) -> Option<Self> {
        let TyShape::Value(value_ty) = ty_shape else {
            return None;
        };

        let def = match value_ty {
            ValueTy::Unit => Self::for_unit(),
            ValueTy::Bool => Self::for_bool(primitives),
            ValueTy::Char => Self::for_char(primitives),
            ValueTy::Int(int_ty) => Self::for_int(*int_ty, primitives),
            ValueTy::Float(float_ty) => Self::for_float(*float_ty, primitives),
            ValueTy::Fn(_) | ValueTy::Ptr(_) | ValueTy::ManyPtr(_) => Self::for_ptr(primitives),
            _ => return None,
        };

        Some(def)
    }

    pub fn for_unit() -> Self {
        Self {
            size: 0,
            align: 1,
            kind: TyKind::Basic,
        }
    }

    pub fn for_bool(primitives: &PrimitiveTyData) -> Self {
        Self {
            size: 1,
            align: primitives.i8_align,
            kind: TyKind::Basic,
        }
    }

    pub fn for_char(primitives: &PrimitiveTyData) -> Self {
        Self {
            size: 1,
            align: primitives.i32_align,
            kind: TyKind::Basic,
        }
    }

    pub fn for_int(int_ty: IntTy, primitives: &PrimitiveTyData) -> Self {
        let (size, align) = match int_ty {
            IntTy::I8 | IntTy::U8 => (1, primitives.i8_align),
            IntTy::I16 | IntTy::U16 => (2, primitives.i16_align),
            IntTy::I32 | IntTy::U32 => (4, primitives.i32_align),
            IntTy::I64 | IntTy::U64 => (8, primitives.i64_align),
            IntTy::I128 | IntTy::U128 => (16, primitives.i128_align),
            IntTy::Isize | IntTy::Usize => (primitives.ptr_size, primitives.ptr_align),
        };

        Self {
            size,
            align,
            kind: TyKind::Basic,
        }
    }

    pub fn for_float(float_ty: FloatTy, primitives: &PrimitiveTyData) -> Self {
        let (size, align) = match float_ty {
            FloatTy::F32 => (4, primitives.f32_align),
            FloatTy::F64 => (8, primitives.f64_align),
        };

        Self {
            size,
            align,
            kind: TyKind::Basic,
        }
    }

    pub fn for_ptr(primitives: &PrimitiveTyData) -> Self {
        Self {
            size: primitives.ptr_size,
            align: primitives.ptr_align,
            kind: TyKind::Basic,
        }
    }

    #[inline]
    pub fn is_zero_sized(&self) -> bool {
        self.size == 0
    }
}
