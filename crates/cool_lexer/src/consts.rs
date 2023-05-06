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
            $($kw_idx:literal: $kw:ident,)+
        }
        Primitives {
            $($primitive_idx:literal: $primitive:ident,)+
        }
        Extra {
            $($extra_idx:literal: $extra_repr:literal as $extra:ident,)+
        }
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
                        symbols.insert_known([<KW_ $kw:upper>], stringify!($kw));
                    )+

                    // Primitives
                    $(
                        symbols.insert_known([<$primitive:upper>], stringify!($primitive));
                    )+

                    // Extra
                    $(
                        symbols.insert_known($extra, $extra_repr);
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
         1: alias,
         2: as,
         3: break,
         4: continue,
         5: crate,
         6: defer,
         7: else,
         8: enum,
         9: export,
        10: extern,
        11: false,
        12: fn,
        13: if,
        14: loop,
        15: module,
        16: mut,
        17: return,
        18: self,
        19: struct,
        20: super,
        21: true,
        22: use,
        23: while,
    }

    Primitives {
        24: i8,
        25: i16,
        26: i32,
        27: i64,
        28: i128,
        29: isize,

        30: u8,
        31: u16,
        32: u32,
        33: u64,
        34: u128,
        35: usize,

        36: f32,
        37: f64,

        38: char,
        39: bool,
    }

    Extra {
        40: "" as EMPTY,
        41: "_" as WILDCARD,

        42: "C" as ABI_C,
        43: "Cool" as ABI_COOL,

        44: "main" as MAIN,
    }
}
