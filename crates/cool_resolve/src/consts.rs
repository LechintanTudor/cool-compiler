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
            use crate::resolve::{ItemId, ResolveTable};

            $(
                pub const $item_ident: ItemId = unsafe { ItemId::new_unchecked($item_idx) };
            )+

            pub fn insert_builtins(resolve: &mut ResolveTable) {
                use cool_lexer::symbols::sym;

                $(
                    resolve.insert_builtin_item($item_ident, sym::$item_ident);
                )+
            }
        }

        #[allow(dead_code)]
        pub mod tys {
            use crate::ty::*;

            pub const UNIT: TyId = unsafe { TyId::new_unchecked(1) };
            $(
                pub const $item_ident: TyId = unsafe { TyId::new_unchecked($item_idx) };
            )+
            $(
                pub const $nonitem_ident: TyId = unsafe { TyId::new_unchecked($nonitem_idx) };
            )+

            pub fn insert_builtins(tys: &mut TyTable) {
                use super::itm;

                tys.insert_builtin(UNIT, TyKind::Unit);
                $(
                    tys.insert_builtin_item(itm::$item_ident, $item_ident, $item_value);
                )+
                $(
                    tys.insert_builtin($nonitem_ident, $nonitem_value);
                )+
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
    }

    Nonitems {
        16: (INFERRED_INT, TyKind::Int(IntTy::Inferred)),
        17: (INFERRED_FLOAT, TyKind::Float(FloatTy::Inferred)),
    }
}
