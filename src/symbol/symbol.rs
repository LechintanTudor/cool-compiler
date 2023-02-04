use crate::symbol::{SymbolTable, SYMBOL_TABLE};
use std::fmt;

pub type SymbolIndex = u32;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Symbol(pub SymbolIndex);

impl Symbol {
    pub fn is_keyword(&self) -> bool {
        *self <= kw::sym::WHILE
    }

    pub fn is_bool_literal(&self) -> bool {
        *self == kw::sym::FALSE || *self == kw::sym::TRUE
    }

    pub fn is_known_suffix(&self) -> bool {
        *self >= ident::sym::I8 && *self <= ident::sym::F64
    }

    pub fn is_wildcard(&self) -> bool {
        *self == ident::sym::WILDCARD
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", SYMBOL_TABLE.get(*self))
    }
}

pub fn intern_symbols(symbols: &mut SymbolTable) {
    kw::sym::intern(symbols);
    ident::sym::intern(symbols);
}

macro_rules! kw_module {
    { $($kw:ident => $idx:literal,)+ } => {
        pub mod kw {
            use crate::lexer::TokenKind;
            use crate::symbol::{Symbol, SymbolTable};
            use paste::paste;

            pub mod sym {
                use super::*;

                paste! {
                    $(
                        pub const [<$kw:upper>]: Symbol = Symbol($idx);
                    )+
                }

                pub fn intern(symbols: &mut SymbolTable) {
                    $(
                        assert_eq!(symbols.insert(stringify!($kw)), Symbol($idx));
                    )+
                }
            }

            paste! {
                $(
                    pub const [<$kw:upper>]: TokenKind = TokenKind::Keyword(Symbol($idx));
                )+
            }
        }
    };
}

macro_rules! ident_module {
    { $($ident:ident => $idx:literal,)+ } => {
        pub mod ident {
            use crate::lexer::TokenKind;
            use crate::symbol::{Symbol, SymbolTable};
            use paste::paste;

            pub mod sym {
                use super::*;

                paste! {
                    $(
                        pub const [<$ident:upper>]: Symbol = Symbol($idx);
                    )+
                }

                pub const WILDCARD: Symbol = Symbol(27);

                pub fn intern(symbols: &mut SymbolTable) {
                    $(
                        assert_eq!(symbols.insert(stringify!($ident)), Symbol($idx));
                    )+
                }
            }

            paste! {
                $(
                    pub const [<$ident:upper>]: TokenKind = TokenKind::Ident(Symbol($idx));
                )+
            }

            pub const WILDCARD: TokenKind = TokenKind::Ident(sym::WILDCARD);
        }
    };
}

kw_module! {
    break => 0,
    continue => 1,
    defer => 2,
    else => 3,
    enum => 4,
    export => 5,
    false => 6,
    fn => 7,
    if => 8,
    import => 9,
    module => 10,
    mut => 11,
    struct => 12,
    true => 13,
    while => 14,
}

ident_module! {
    i8 => 15,
    i16 => 16,
    i32 => 17,
    i64 => 18,

    u8 => 19,
    u16 => 20,
    u32 => 21,
    u64 => 22,

    isize => 23,
    usize => 24,

    f32 => 25,
    f64 => 26,
}
