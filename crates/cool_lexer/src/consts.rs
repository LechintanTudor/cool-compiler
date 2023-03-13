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
        crate => 3,
        defer => 4,
        else => 5,
        enum => 6,
        export => 7,
        false => 8,
        fn => 9,
        if => 10,
        module => 11,
        mut => 12,
        return => 13,
        self => 14,
        struct => 15,
        super => 16,
        true => 17,
        use => 18,
        while => 19,
    },
    Primitives {
        i8 => 20,
        i16 => 21,
        i32 => 22,
        i64 => 23,
        i128 => 24,
        isize => 25,

        u8 => 26,
        u16 => 27,
        u32 => 28,
        u64 => 29,
        u128 => 30,
        usize => 31,

        f32 => 32,
        f64 => 33,
    },
    Extra {
        EMPTY: "" => 34,
        WILDCARD: "_" => 35,
        GLOB: "*" => 36,
    },
}
