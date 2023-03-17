macro_rules! builtins {
    {
        Items {
            $($item_ident:ident => ($item_idx:tt, $item_value:expr),)+
        }
        Nonitems {
            $($nonitem_ident:ident => ($nonitem_idx:tt, $nonitem_value:expr),)+
        }
    } => {
        #[allow(dead_code)]
        pub mod itm {
            use crate::item::{ItemId, ItemTable};

            $(
                pub const $item_ident: ItemId = unsafe { ItemId::new_unchecked($item_idx) };
            )+

            pub fn insert_builtins(items: &mut ItemTable) {
                use cool_lexer::symbols::sym;

                $(
                    items.insert_builtin($item_ident, sym::$item_ident);
                )+
            }
        }

        #[allow(dead_code)]
        pub mod tys {
            use crate::ty::*;

            $(
                pub const $item_ident: TyId = unsafe { TyId::new_unchecked($item_idx) };
            )+
            $(
                pub const $nonitem_ident: TyId = unsafe { TyId::new_unchecked($nonitem_idx) };
            )+

            pub fn insert_builtins(tys: &mut TyTable) {
                use super::itm;

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
        I8 => (1, TyKind::Int(IntTy::I8)),
        I16 => (2, TyKind::Int(IntTy::I16)),
        I32 => (3, TyKind::Int(IntTy::I32)),
        I64 => (4, TyKind::Int(IntTy::I64)),
        I128 => (5, TyKind::Int(IntTy::I128)),
        ISIZE => (6, TyKind::Int(IntTy::Isize)),

        U8 => (7, TyKind::Int(IntTy::U8)),
        U16 => (8, TyKind::Int(IntTy::U16)),
        U32 => (9, TyKind::Int(IntTy::U32)),
        U64 => (10, TyKind::Int(IntTy::U64)),
        U128 => (11, TyKind::Int(IntTy::U128)),
        USIZE => (12, TyKind::Int(IntTy::Usize)),

        F32 => (13, TyKind::Float(FloatTy::F32)),
        F64 => (14, TyKind::Float(FloatTy::F64)),
    }

    Nonitems {
        UNIT => (15, TyKind::Unit),
    }
}
