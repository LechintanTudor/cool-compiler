use crate::symbol::SYMBOL_TABLE;
use std::fmt;

pub type SymbolIndex = u32;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Symbol(pub SymbolIndex);

impl Symbol {
    pub fn is_keyword(&self) -> bool {
        *self <= sym::WHILE
    }

    pub fn is_bool_literal(&self) -> bool {
        *self == sym::FALSE || *self == sym::TRUE
    }

    pub fn is_known_suffix(&self) -> bool {
        *self >= sym::I8 && *self <= sym::F64
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", SYMBOL_TABLE.get(*self))
    }
}

macro_rules! define_symbols {
    { $($kw:ident => $idx:literal,)+ } => {
        pub mod sym {
            use crate::symbol::{Symbol, SymbolTable};
            use paste::paste;

            paste! {
                pub const ALL: &[&str] = &[$(
                    stringify!([<$kw:lower>]),
                )+];
            }

            $(
                pub const $kw: Symbol = Symbol($idx);
            )+

            pub(crate) fn intern_keywords(symbols: &mut SymbolTable) {
                paste! {
                    $({
                        let symbol_str = stringify!([<$kw:lower>]);
                        let symbol = symbols.insert(symbol_str);
                        assert!(symbol.0 == $idx);
                    })+
                }
            }
        }

        pub mod kw {
            use crate::lexer::TokenKind;
            use crate::symbol::Symbol;

            $(
                pub const $kw: TokenKind = TokenKind::Keyword(Symbol($idx));
            )+
        }
    };
}

define_symbols! {
    // Keywords
    BREAK => 0,
    CONTINUE => 1,
    DEFER => 2,
    ELSE => 3,
    ENUM => 4,
    EXPORT => 5,
    FALSE => 6,
    FN => 7,
    IF => 8,
    IMPORT => 9,
    MODULE => 10,
    MUT => 11,
    STRUCT => 12,
    TRUE => 13,
    WHILE => 14,

    // Primitives
    I8 => 15,
    I16 => 16,
    I32 => 17,
    I64 => 18,

    U8 => 19,
    U16 => 20,
    U32 => 21,
    U64 => 22,

    ISIZE => 23,
    USIZE => 24,

    F32 => 25,
    F64 => 26,
}
