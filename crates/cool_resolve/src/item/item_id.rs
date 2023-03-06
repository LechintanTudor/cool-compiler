use cool_arena::{handle_newtype, SliceHandle};
use cool_lexer::symbols::Symbol;

handle_newtype!(ItemId wraps SliceHandle<Symbol>; Debug);

impl ItemId {
    #[inline]
    pub const fn is_builtin(&self) -> bool {
        self.0.index() <= itm::F64.0.index()
    }
}

macro_rules! builtins {
    { $($ident:ident => $idx:tt,)+ } => {
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
    };
}

builtins! {
    I8 => 1,
    I16 => 2,
    I32 => 3,
    I64 => 4,

    U8 => 5,
    U16 => 6,
    U32 => 7,
    U64 => 8,

    ISIZE => 9,
    USIZE => 10,

    F32 => 11,
    F64 => 12,
}
