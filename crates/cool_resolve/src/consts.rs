macro_rules! builtins {
    {
        Nonitems {
            $($nonitem_ident:ident => ($nonitem_idx:tt, $nonitem_value:expr),)+
        }
        Items {
            $($item_ident:ident => ($item_idx:tt, $item_value:expr),)+
        }
    } => {
        #[allow(dead_code)]
        pub mod itm {
            use crate::{ItemId, ResolveTable};

            $(
                pub const $item_ident: ItemId = unsafe { ItemId::new_unchecked($item_idx) };
            )+

            pub fn add_builtins(symbols: &mut ResolveTable) {
                use cool_lexer::symbols::sym;

                $(
                    symbols.add_builtin_item($item_ident, sym::$item_ident);
                )+
            }
        }

        #[allow(dead_code)]
        pub mod tys {
            use crate::ty::*;

            $(
                pub const $nonitem_ident: TyId = unsafe { TyId::new_unchecked($nonitem_idx) };
            )+
            $(
                pub const $item_ident: TyId = unsafe { TyId::new_unchecked($item_idx) };
            )+

            pub fn insert_builtins(tys: &mut TyTable) {
                use super::itm;

                $(
                    tys.insert_builtin($nonitem_ident, $nonitem_value);
                )+
                $(
                    tys.insert_builtin_item(itm::$item_ident, $item_ident, $item_value);
                )+
            }
        }
    };
}

builtins! {
    Nonitems {
        UNIT => (1, TyKind::Unit),
    }

    Items {
        I8 => (2, TyKind::Int(IntTy::I8)),
        I16 => (3, TyKind::Int(IntTy::I16)),
        I32 => (4, TyKind::Int(IntTy::I32)),
        I64 => (5, TyKind::Int(IntTy::I64)),
        I128 => (6, TyKind::Int(IntTy::I128)),
        ISIZE => (7, TyKind::Int(IntTy::Isize)),

        U8 => (8, TyKind::Int(IntTy::U8)),
        U16 => (9, TyKind::Int(IntTy::U16)),
        U32 => (10, TyKind::Int(IntTy::U32)),
        U64 => (11, TyKind::Int(IntTy::U64)),
        U128 => (12, TyKind::Int(IntTy::U128)),
        USIZE => (13, TyKind::Int(IntTy::Usize)),

        F32 => (14, TyKind::Float(FloatTy::F32)),
        F64 => (15, TyKind::Float(FloatTy::F64)),
    }
}
