use crate::{
    FloatTy, InferTy, IntTy, ItemTy, ManyPtrTy, ResolveTy, TyArena, TyContext, TyDef, TyId,
    TyShape, ValueTy,
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
    pub fn blank(tys: &mut TyArena) -> Self {
        let diverge = TyId::from(tys.insert(ResolveTy {
            shape: TyShape::Diverge,
            def: TyDef::Undefined,
        }));

        Self {
            infer: diverge,
            infer_number: diverge,
            infer_int: diverge,
            infer_float: diverge,
            infer_empty_array: diverge,
            module: diverge,
            ty: diverge,
            unit: diverge,
            bool: diverge,
            char: diverge,
            c_str: diverge,
            i8: diverge,
            i16: diverge,
            i32: diverge,
            i64: diverge,
            i128: diverge,
            isize: diverge,
            u8: diverge,
            u16: diverge,
            u32: diverge,
            u64: diverge,
            u128: diverge,
            usize: diverge,
            f32: diverge,
            f64: diverge,
            diverge,
        }
    }

    pub fn new(tys: &mut TyContext) -> Self {
        let i8_ty = tys.insert(TyShape::Value(IntTy::I8.into()));
        let c_str = tys.insert(TyShape::Value(ValueTy::ManyPtr(ManyPtrTy {
            pointee: i8_ty,
            is_mutable: false,
        })));

        Self {
            // Inferred
            infer: tys.insert(TyShape::Infer(InferTy::Any)),
            infer_number: tys.insert(TyShape::Infer(InferTy::Number)),
            infer_int: tys.insert(TyShape::Infer(InferTy::Int)),
            infer_float: tys.insert(TyShape::Infer(InferTy::Float)),
            infer_empty_array: tys.insert(TyShape::Infer(InferTy::Array)),

            // Items
            module: tys.insert(TyShape::Item(ItemTy::Module)),
            ty: tys.insert(TyShape::Item(ItemTy::Ty)),

            // Non-number primitives
            unit: tys.insert_value(ValueTy::Unit),
            bool: tys.insert_value(ValueTy::Bool),
            char: tys.insert_value(ValueTy::Char),
            c_str,

            // Signed integers
            i8: tys.insert_value(IntTy::I8),
            i16: tys.insert_value(IntTy::I16),
            i32: tys.insert_value(IntTy::I32),
            i64: tys.insert_value(IntTy::I64),
            i128: tys.insert_value(IntTy::I128),
            isize: tys.insert_value(IntTy::Isize),

            // Unsigned integers
            u8: tys.insert_value(IntTy::U8),
            u16: tys.insert_value(IntTy::U16),
            u32: tys.insert_value(IntTy::U32),
            u64: tys.insert_value(IntTy::U64),
            u128: tys.insert_value(IntTy::U128),
            usize: tys.insert_value(IntTy::Usize),

            // Floats
            f32: tys.insert_value(FloatTy::F32),
            f64: tys.insert_value(FloatTy::F64),

            // Diverge
            diverge: tys.insert(TyShape::Diverge),
        }
    }
}
