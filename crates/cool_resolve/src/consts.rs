macro_rules! builtins {
    { $($ident:ident => ($idx:tt, $value:expr),)+ } => {
        #[allow(dead_code)]
        pub mod itm {
            use crate::item::{ItemId, ItemTable};

            $(
                pub const $ident: ItemId = unsafe { ItemId::new_unchecked($idx) };
            )+

            pub fn insert_builtins(items: &mut ItemTable) {
                use cool_lexer::symbols::sym;

                $(
                    items.insert_builtin($ident, sym::$ident);
                )+
            }
        }

        #[allow(dead_code)]
        pub mod tys {
            use crate::ty::*;

            $(
                pub const $ident: TyId = unsafe { TyId::new_unchecked($idx) };
            )+

            pub fn insert_builtins(tys: &mut TyTable) {
                $(
                    tys.insert_builtin(super::itm::$ident, $ident, $value);
                )+
            }
        }
    };
}

builtins! {
    I8 => (1, TyKind::Int(IntTy::I8)),
    I16 => (2, TyKind::Int(IntTy::I16)),
    I32 => (3, TyKind::Int(IntTy::I32)),
    I64 => (4, TyKind::Int(IntTy::I64)),

    U8 => (5, TyKind::Uint(UintTy::U8)),
    U16 => (6, TyKind::Uint(UintTy::U16)),
    U32 => (7, TyKind::Uint(UintTy::U32)),
    U64 => (8, TyKind::Uint(UintTy::U64)),

    ISIZE => (9, TyKind::Int(IntTy::Isize)),
    USIZE => (10, TyKind::Uint(UintTy::Usize)),

    F32 => (11, TyKind::Float(FloatTy::F32)),
    F64 => (12, TyKind::Float(FloatTy::F64)),
}
