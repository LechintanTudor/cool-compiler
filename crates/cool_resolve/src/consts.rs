use crate::context::ResolveContext;
use crate::{AnyTy, FloatTy, InferTy, IntTy, ItemTy, PrimitiveTyProps, PtrTy, ValueTy};
use cool_lexer::symbols::sym;

macro_rules! builtins {
    {
        Items {
            $($item_idx:tt: ($item_ident:ident, $item_value:expr),)+
        }
        Nonitems {
            $($nonitem_idx:tt: ($nonitem_ident:ident, $nonitem_value:expr),)+
        }
    } => {
        #[allow(dead_code)]
        pub mod itm {
            use crate::ItemId;

            $(
                pub const $item_ident: ItemId = unsafe { ItemId::new_unchecked($item_idx) };
            )+
        }

        #[allow(dead_code)]
        pub mod tys {
            use crate::TyId;

            pub const UNIT: TyId = unsafe { TyId::new_unchecked(1) };
            $(
                pub const $item_ident: TyId = unsafe { TyId::new_unchecked($item_idx) };
            )+
            $(
                pub const $nonitem_ident: TyId = unsafe { TyId::new_unchecked($nonitem_idx) };
            )+
        }

        impl ResolveContext {
            pub fn new(primitives: PrimitiveTyProps) -> Self {
                let mut resolve = ResolveContext::empty(primitives);
                resolve.insert_root_module(sym::EMPTY).unwrap();
                resolve.insert_builtin_ty(tys::UNIT, AnyTy::Value(ValueTy::Unit));

                $(
                    resolve.insert_builtin_ty_item(
                        sym::$item_ident,
                        itm::$item_ident,
                        tys::$item_ident,
                        $item_value,
                    );
                )+

                $(
                    resolve.insert_builtin_ty(tys::$nonitem_ident, $nonitem_value);
                )+

                resolve
            }
        }
    };
}

builtins! {
    Items {
         2: (I8, AnyTy::Value(ValueTy::Int(IntTy::I8))),
         3: (I16, AnyTy::Value(ValueTy::Int(IntTy::I16))),
         4: (I32, AnyTy::Value(ValueTy::Int(IntTy::I32))),
         5: (I64, AnyTy::Value(ValueTy::Int(IntTy::I64))),
         6: (I128, AnyTy::Value(ValueTy::Int(IntTy::I128))),
         7: (ISIZE, AnyTy::Value(ValueTy::Int(IntTy::Isize))),

         8: (U8, AnyTy::Value(ValueTy::Int(IntTy::U8))),
         9: (U16, AnyTy::Value(ValueTy::Int(IntTy::U16))),
        10: (U32, AnyTy::Value(ValueTy::Int(IntTy::U32))),
        11: (U64, AnyTy::Value(ValueTy::Int(IntTy::U64))),
        12: (U128, AnyTy::Value(ValueTy::Int(IntTy::U128))),
        13: (USIZE, AnyTy::Value(ValueTy::Int(IntTy::Usize))),

        14: (F32, AnyTy::Value(ValueTy::Float(FloatTy::F32))),
        15: (F64, AnyTy::Value(ValueTy::Float(FloatTy::F64))),

        16: (BOOL, AnyTy::Value(ValueTy::Bool)),
        17: (CHAR, AnyTy::Value(ValueTy::Char)),
    }

    Nonitems {
        18: (INFER, AnyTy::Infer(InferTy::Any)),
        19: (INFER_NUMBER, AnyTy::Infer(InferTy::Number)),
        20: (INFER_INT, AnyTy::Infer(InferTy::Int)),
        21: (INFER_FLOAT, AnyTy::Infer(InferTy::Float)),
        22: (INFER_EMPTY_ARRAY, AnyTy::Infer(InferTy::EmptyArray)),

        23: (DIVERGE, AnyTy::Diverge),

        24: (MODULE, AnyTy::Item(ItemTy::Module)),
        25: (TY, AnyTy::Item(ItemTy::Ty)),
        26: (
            C_STR,
            AnyTy::Value(ValueTy::Ptr(PtrTy { is_mutable: false, pointee: tys::I8 }))
        ),
    }
}
