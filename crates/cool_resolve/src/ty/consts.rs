use crate::{FloatTy, IntTy, PrimitiveTyData, TyArena, TyId, ValueTy};

#[derive(Clone, Copy, Debug)]
pub struct TyConsts {
    pub unit: TyId,
    pub i8: TyId,
    pub i16: TyId,
    pub i32: TyId,
    pub i64: TyId,
    pub i128: TyId,
    pub isize: TyId,
    pub u8: TyId,
    pub u16: TyId,
    pub u32: TyId,
    pub u64: TyId,
    pub u128: TyId,
    pub usize: TyId,
    pub f32: TyId,
    pub f64: TyId,
}

impl TyConsts {
    pub fn new(tys: &mut TyArena, primitives: &PrimitiveTyData) -> Self {
        let mut insert_ty = |ty: ValueTy| -> TyId {
            let ty = ty.to_resolve_ty(primitives);
            let internal_ty_id = tys.get_or_insert(ty);
            TyId::new(tys.get(internal_ty_id).unwrap())
        };

        Self {
            unit: insert_ty(ValueTy::Unit),
            i8: insert_ty(IntTy::I8.into()),
            i16: insert_ty(IntTy::I16.into()),
            i32: insert_ty(IntTy::I32.into()),
            i64: insert_ty(IntTy::I64.into()),
            i128: insert_ty(IntTy::I128.into()),
            isize: insert_ty(IntTy::Isize.into()),
            u8: insert_ty(IntTy::U8.into()),
            u16: insert_ty(IntTy::U16.into()),
            u32: insert_ty(IntTy::U32.into()),
            u64: insert_ty(IntTy::U64.into()),
            u128: insert_ty(IntTy::U128.into()),
            usize: insert_ty(IntTy::Usize.into()),
            f32: insert_ty(FloatTy::F32.into()),
            f64: insert_ty(FloatTy::F64.into()),
        }
    }
}
