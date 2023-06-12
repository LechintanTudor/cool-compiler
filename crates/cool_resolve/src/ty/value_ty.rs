use crate::{
    resolve_fields_size_align, AnyTy, ArrayTy, FloatTy, FnTy, IntTy, ManyPtrTy, PrimitiveTyData,
    PtrTy, ResolveTy, SliceTy, StructTy, TupleTy,
};
use derive_more::From;

#[derive(Clone, Eq, PartialEq, Hash, From, Debug)]
pub enum ValueTy {
    Unit,
    Int(IntTy),
    Float(FloatTy),
    Array(ArrayTy),
    Tuple(TupleTy),
    Struct(StructTy),
    Fn(FnTy),
    Ptr(PtrTy),
    ManyPtr(ManyPtrTy),
    Slice(SliceTy),
}

impl ValueTy {
    pub fn to_resolve_ty(self, primitives: &PrimitiveTyData) -> ResolveTy {
        match self {
            ValueTy::Unit => {
                ResolveTy {
                    size: 0,
                    align: 1,
                    ty: AnyTy::Value(self),
                }
            }
            ValueTy::Int(int_ty) => int_ty.to_resolve_ty(&primitives),
            ValueTy::Float(float_ty) => float_ty.to_resolve_ty(&primitives),
            ValueTy::Array(array_ty) => {
                let elem_size = array_ty.elem.get_size();
                let elem_align = array_ty.elem.get_align();

                ResolveTy {
                    size: elem_size * array_ty.len,
                    align: elem_align,
                    ty: AnyTy::Value(ValueTy::Array(array_ty)),
                }
            }
            ValueTy::Tuple(mut tuple_ty) => {
                if tuple_ty.fields.is_empty() {
                    return ResolveTy {
                        size: 0,
                        align: 1,
                        ty: AnyTy::Value(ValueTy::Unit),
                    };
                }

                let (size, align) = resolve_fields_size_align(&mut tuple_ty.fields);

                ResolveTy {
                    size,
                    align,
                    ty: AnyTy::Value(ValueTy::Tuple(tuple_ty)),
                }
            }
            ValueTy::Struct(_) => {
                ResolveTy {
                    size: 0,
                    align: 1,
                    ty: AnyTy::Value(self),
                }
            }
            ValueTy::Fn(_) | ValueTy::Ptr(_) | ValueTy::ManyPtr(_) => {
                ResolveTy {
                    size: primitives.ptr_size,
                    align: primitives.ptr_align,
                    ty: AnyTy::Value(self),
                }
            }
            ValueTy::Slice(mut slice_ty) => {
                let (size, align) = resolve_fields_size_align(&mut slice_ty.fields);

                ResolveTy {
                    size,
                    align,
                    ty: AnyTy::Value(self),
                }
            }
        }
    }
}
