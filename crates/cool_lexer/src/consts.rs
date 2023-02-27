use crate::symbols::Symbol;
use std::fmt;

impl Symbol {
    #[inline]
    pub fn is_keyword(&self) -> bool {
        *self <= sym::KW_WHILE
    }

    #[inline]
    pub fn is_bool_literal(&self) -> bool {
        *self == sym::KW_FALSE || *self == sym::KW_TRUE
    }

    #[inline]
    pub fn is_known_suffix(&self) -> bool {
        *self >= sym::I8 && *self <= sym::F64
    }

    #[inline]
    pub fn is_wildcard(&self) -> bool {
        *self == sym::WILDCARD
    }

    #[inline]
    pub fn as_str(&self) -> &'static str {
        if *self <= sym::WILDCARD {
            sym::ALL_REPRS[self.as_usize()]
        } else {
            Symbol::as_str_from_symbol_table(*self)
        }
    }
}

impl Default for Symbol {
    #[inline]
    fn default() -> Self {
        sym::EMPTY
    }
}

impl fmt::Display for Symbol {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl fmt::Debug for Symbol {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.as_str())
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
        #[allow(dead_code)]
        pub mod sym {
            use crate::symbols::{Symbol, SymbolTable};
            use paste::paste;

            pub const ALL_REPRS: &[&str] = &[
                "",
                $(stringify!($kw),)+
                $(stringify!($primitive),)+
                $($extra_repr,)+
            ];

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

        #[allow(dead_code)]
        pub mod tk {
            use crate::consts::sym;
            use crate::tokens::TokenKind;
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
        use => 10,
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
        EMPTY: "" => 28,
        WILDCARD: "_" => 29,
    },
}
