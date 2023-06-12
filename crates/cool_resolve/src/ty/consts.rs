use crate::{
    AnyTy, FloatTy, InferTy, IntTy, ItemTy, ManyPtrTy, PrimitiveTyData, TyArena, TyId, ValueTy,
};

#[derive(Clone, Copy, Debug)]
pub struct TyConsts {
    // Inferred
    pub infer: TyId,
    pub infer_number: TyId,
    pub infer_int: TyId,
    pub infer_float: TyId,
    pub infer_empty_array: TyId,

    // Items
    pub module: TyId,
    pub ty: TyId,

    // Non-number primitives
    pub unit: TyId,
    pub bool: TyId,
    pub char: TyId,
    pub c_str: TyId,

    // Signed integers
    pub i8: TyId,
    pub i16: TyId,
    pub i32: TyId,
    pub i64: TyId,
    pub i128: TyId,
    pub isize: TyId,

    // Unsigned integers
    pub u8: TyId,
    pub u16: TyId,
    pub u32: TyId,
    pub u64: TyId,
    pub u128: TyId,
    pub usize: TyId,

    // Floats
    pub f32: TyId,
    pub f64: TyId,

    // Diverge
    pub diverge: TyId,
}

impl TyConsts {
    pub fn new(tys: &mut TyArena, primitives: &PrimitiveTyData) -> Self {
        let mut insert_ty = |ty: AnyTy| -> TyId {
            let ty = ty.to_resolve_ty(primitives);
            let internal_ty_id = tys.get_or_insert(ty);
            TyId::new(tys.get(internal_ty_id).unwrap())
        };

        let char = insert_ty(AnyTy::Value(ValueTy::Bool));
        let c_str = insert_ty(AnyTy::Value(ValueTy::ManyPtr(ManyPtrTy {
            pointee: char,
            is_mutable: false,
        })));

        Self {
            // Inferred
            infer: insert_ty(AnyTy::Infer(InferTy::Any)),
            infer_number: insert_ty(AnyTy::Infer(InferTy::Number)),
            infer_int: insert_ty(AnyTy::Infer(InferTy::Int)),
            infer_float: insert_ty(AnyTy::Infer(InferTy::Float)),
            infer_empty_array: insert_ty(AnyTy::Infer(InferTy::EmptyArray)),

            // Items
            module: insert_ty(AnyTy::Item(ItemTy::Module)),
            ty: insert_ty(AnyTy::Item(ItemTy::Ty)),

            // Non-number primitives
            unit: insert_ty(AnyTy::Value(ValueTy::Unit)),
            bool: insert_ty(AnyTy::Value(ValueTy::Bool)),
            char,
            c_str,

            // Signed integers
            i8: insert_ty(AnyTy::Value(IntTy::I8.into())),
            i16: insert_ty(AnyTy::Value(IntTy::I16.into())),
            i32: insert_ty(AnyTy::Value(IntTy::I32.into())),
            i64: insert_ty(AnyTy::Value(IntTy::I64.into())),
            i128: insert_ty(AnyTy::Value(IntTy::I128.into())),
            isize: insert_ty(AnyTy::Value(IntTy::Isize.into())),

            // Unsigned integers
            u8: insert_ty(AnyTy::Value(IntTy::U8.into())),
            u16: insert_ty(AnyTy::Value(IntTy::U16.into())),
            u32: insert_ty(AnyTy::Value(IntTy::U32.into())),
            u64: insert_ty(AnyTy::Value(IntTy::U64.into())),
            u128: insert_ty(AnyTy::Value(IntTy::U128.into())),
            usize: insert_ty(AnyTy::Value(IntTy::Usize.into())),

            // Floats
            f32: insert_ty(AnyTy::Value(FloatTy::F32.into())),
            f64: insert_ty(AnyTy::Value(FloatTy::F64.into())),

            // Diverge
            diverge: insert_ty(AnyTy::Diverge),
        }
    }
}
