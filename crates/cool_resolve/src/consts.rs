use crate::context::ResolveContext;
use crate::{FloatTy, InferredTy, IntTy, PointerTy, PrimitiveTyProps, TyKind};
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
                resolve.insert_builtin_ty(tys::UNIT, TyKind::Unit);

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
         2: (I8, TyKind::Int(IntTy::I8)),
         3: (I16, TyKind::Int(IntTy::I16)),
         4: (I32, TyKind::Int(IntTy::I32)),
         5: (I64, TyKind::Int(IntTy::I64)),
         6: (I128, TyKind::Int(IntTy::I128)),
         7: (ISIZE, TyKind::Int(IntTy::Isize)),

         8: (U8, TyKind::Int(IntTy::U8)),
         9: (U16, TyKind::Int(IntTy::U16)),
        10: (U32, TyKind::Int(IntTy::U32)),
        11: (U64, TyKind::Int(IntTy::U64)),
        12: (U128, TyKind::Int(IntTy::U128)),
        13: (USIZE, TyKind::Int(IntTy::Usize)),

        14: (F32, TyKind::Float(FloatTy::F32)),
        15: (F64, TyKind::Float(FloatTy::F64)),

        16: (BOOL, TyKind::Bool),
        17: (CHAR, TyKind::Char),
    }

    Nonitems {
        18: (INFERRED, TyKind::Inferred(InferredTy::Any)),
        19: (INFERRED_NUMBER, TyKind::Inferred(InferredTy::Number)),
        20: (INFERRED_INT, TyKind::Inferred(InferredTy::Int)),
        21: (INFERRED_FLOAT, TyKind::Inferred(InferredTy::Float)),

        22: (MODULE, TyKind::Module),
        23: (C_STR, TyKind::Pointer(PointerTy { is_mutable: false, pointee: tys::I8 })),
    }
}
