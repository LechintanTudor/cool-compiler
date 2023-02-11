use crate::symbol::SYMBOL_TABLE;
use std::fmt;
use std::num::NonZeroU32;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Symbol(pub NonZeroU32);

impl Symbol {
    pub const unsafe fn new_unchecked(index: u32) -> Self {
        Self(NonZeroU32::new_unchecked(index))
    }

    pub fn is_keyword(&self) -> bool {
        *self <= sym::KW_WHILE
    }

    pub fn is_bool_literal(&self) -> bool {
        *self == sym::KW_FALSE || *self == sym::KW_TRUE
    }

    pub fn is_known_suffix(&self) -> bool {
        *self >= sym::I8 && *self <= sym::F64
    }

    pub fn is_wildcard(&self) -> bool {
        *self == sym::WILDCARD
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Optimize for preinterned symbols
        write!(f, "{}", SYMBOL_TABLE.get(*self))
    }
}

macro_rules! define_symbols {
    {
        Keywords {
            $($kw:ident => $kw_idx:literal,)+
        },
        Primitives {
            $($primitive:ident => $primitive_idx:literal,)+
        },
        Extra {
            $($extra:ident: $extra_repr:literal => $extra_idx:literal,)+
        },
    } => {
        pub mod sym {
            use crate::symbol::{Symbol, SymbolTable};
            use paste::paste;

            paste! {
                // Keywords
                $(
                    pub const [<KW_ $kw:upper>]: Symbol
                        = unsafe { Symbol::new_unchecked($kw_idx) };
                )+

                // Primitives
                $(
                    pub const [<$primitive:upper>]: Symbol
                        = unsafe { Symbol::new_unchecked($primitive_idx) };
                )+
            }

            // Extra
            $(
                pub const $extra: Symbol = unsafe { Symbol::new_unchecked($extra_idx) };
            )+

            pub fn intern_symbols(symbols: &mut SymbolTable) {
                paste! {
                    // Keywords
                    $(
                        assert_eq!(symbols.insert(stringify!($kw)), [<KW_ $kw:upper>]);
                    )+

                    // Primitives
                    $(
                        assert_eq!(symbols.insert(stringify!($primitive)), [<$primitive:upper>]);
                    )+

                    // Extra
                    $(
                        assert_eq!(symbols.insert($extra_repr), $extra);
                    )+
                }
            }
        }

        pub mod tk {
            use crate::lexer::TokenKind;
            use crate::symbol::sym;
            use paste::paste;

            paste! {
                // Keywords
                $(
                    pub const [<KW_ $kw:upper>]: TokenKind
                        = TokenKind::Keyword(sym::[<KW_ $kw:upper>]);
                )+

                // Primitives
                $(
                    pub const [<$primitive:upper>]: TokenKind
                        = TokenKind::Ident(sym::[<$primitive:upper>]);
                )+
            }

            // Extra
            $(
                pub const $extra: TokenKind = TokenKind::Ident(sym::$extra);
            )+
        }
    };
}

define_symbols! {
    Keywords {
        break => 1,
        continue => 2,
        defer => 3,
        else => 4,
        enum => 5,
        export => 6,
        false => 7,
        fn => 8,
        if => 9,
        import => 10,
        module => 11,
        mut => 12,
        struct => 13,
        true => 14,
        while => 15,
    },
    Primitives {
        i8 => 16,
        i16 => 17,
        i32 => 18,
        i64 => 19,

        u8 => 20,
        u16 => 21,
        u32 => 22,
        u64 => 23,

        isize => 24,
        usize => 25,

        f32 => 26,
        f64 => 27,
    },
    Extra {
        WILDCARD: "_" => 28,
        ANY_IDENT: "<identifier>" => 29,
    },
}
