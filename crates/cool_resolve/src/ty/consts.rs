use crate::{
    FloatTy, InferTy, IntTy, ItemTy, ManyPtrTy, PrimitiveTyData, TyDef, TyDefs, TyId, TyShape,
    TyShapes, ValueTy,
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
    pub fn new(shapes: &mut TyShapes, defs: &mut TyDefs, primitives: &PrimitiveTyData) -> Self {
        let mut insert_ty_shape = |shape| {
            let ty_id = TyId::from(shapes.insert(shape));

            if let Some(ty_def) = TyDef::for_basic(&ty_id, primitives) {
                defs.insert(ty_id, ty_def);
            }

            ty_id
        };

        let i8_ty = insert_ty_shape(TyShape::from(ValueTy::from(IntTy::I8)));
        let c_str = insert_ty_shape(TyShape::from(ValueTy::ManyPtr(ManyPtrTy {
            pointee: i8_ty,
            is_mutable: false,
        })));

        Self {
            // Inferred
            infer: insert_ty_shape(TyShape::from(InferTy::Any)),
            infer_number: insert_ty_shape(TyShape::from(InferTy::Number)),
            infer_int: insert_ty_shape(TyShape::from(InferTy::Int)),
            infer_float: insert_ty_shape(TyShape::from(InferTy::Float)),
            infer_empty_array: insert_ty_shape(TyShape::from(InferTy::Array)),

            // Items
            module: insert_ty_shape(TyShape::from(ItemTy::Module)),
            ty: insert_ty_shape(TyShape::from(ItemTy::Ty)),

            // Non-number primitives
            unit: insert_ty_shape(TyShape::from(ValueTy::Unit)),
            bool: insert_ty_shape(TyShape::from(ValueTy::Bool)),
            char: insert_ty_shape(TyShape::from(ValueTy::Char)),
            c_str,

            // Signed integers
            i8: insert_ty_shape(TyShape::from(ValueTy::from(IntTy::I8))),
            i16: insert_ty_shape(TyShape::from(ValueTy::from(IntTy::I16))),
            i32: insert_ty_shape(TyShape::from(ValueTy::from(IntTy::I32))),
            i64: insert_ty_shape(TyShape::from(ValueTy::from(IntTy::I64))),
            i128: insert_ty_shape(TyShape::from(ValueTy::from(IntTy::I128))),
            isize: insert_ty_shape(TyShape::from(ValueTy::from(IntTy::Isize))),

            // Unsigned integers
            u8: insert_ty_shape(TyShape::from(ValueTy::from(IntTy::U8))),
            u16: insert_ty_shape(TyShape::from(ValueTy::from(IntTy::U16))),
            u32: insert_ty_shape(TyShape::from(ValueTy::from(IntTy::U32))),
            u64: insert_ty_shape(TyShape::from(ValueTy::from(IntTy::U64))),
            u128: insert_ty_shape(TyShape::from(ValueTy::from(IntTy::U128))),
            usize: insert_ty_shape(TyShape::from(ValueTy::from(IntTy::Usize))),

            // Floats
            f32: insert_ty_shape(TyShape::from(ValueTy::from(FloatTy::F32))),
            f64: insert_ty_shape(TyShape::from(ValueTy::from(FloatTy::F64))),

            // Diverge
            diverge: insert_ty_shape(TyShape::Diverge),
        }
    }
}
