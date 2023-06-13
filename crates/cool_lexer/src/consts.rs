macro_rules! define_symbols {
    {
        Keywords {
            $($kw_idx:literal: $kw:ident,)+
        },
        Primitives {
            $($primitive_idx:literal: $primitive:ident,)+
        },
        Digits {
            $($digit_idx:literal: $digit_repr:literal as $digit:ident,)+
        },
        Extra {
            $($extra_idx:literal: $extra_repr:literal as $extra:ident,)+
        },
    } => {
        #[allow(dead_code)]
        pub mod sym {
            use crate::symbols::{Symbol, SymbolTable};
            use paste::paste;

            pub const ALL_REPRS: &[&str] = &[
                $(stringify!($kw),)+
                $(stringify!($primitive),)+
                $($digit_repr,)+
                $($extra_repr,)+
            ];

            pub const ALL_DIGITS: &[Symbol] = &[
                $($digit,)+
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

            $(
                pub const $digit: Symbol = unsafe { Symbol::new_unchecked($digit_idx) };
            )+

            // Extra
            $(
                pub const $extra: Symbol = unsafe { Symbol::new_unchecked($extra_idx) };
            )+

            pub fn intern_symbols(symbols: &mut SymbolTable) {
                paste! {
                    // Keywords
                    $(
                        symbols.insert_checked([<KW_ $kw:upper>], stringify!($kw));
                    )+

                    // Primitives
                    $(
                        symbols.insert_checked([<$primitive:upper>], stringify!($primitive));
                    )+

                    // Digits
                    $(
                        symbols.insert_checked($digit, $digit_repr);
                    )+

                    // Extra
                    $(
                        symbols.insert_checked($extra, $extra_repr);
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
         1: as,
         2: break,
         3: continue,
         4: crate,
         5: defer,
         6: else,
         7: enum,
         8: export,
         9: extern,
        10: false,
        11: fn,
        12: for,
        13: if,
        14: loop,
        15: module,
        16: mut,
        17: return,
        18: self,
        19: struct,
        20: super,
        21: true,
        22: type,
        23: use,
        24: while,
    },
    Primitives {
        25: i8,
        26: i16,
        27: i32,
        28: i64,
        29: i128,
        30: isize,

        31: u8,
        32: u16,
        33: u32,
        34: u64,
        35: u128,
        36: usize,

        37: f32,
        38: f64,

        39: char,
        40: bool,
    },
    Digits {
        41: "0" as DIGIT_0,
        42: "1" as DIGIT_1,
        43: "2" as DIGIT_2,
        44: "3" as DIGIT_3,
        45: "4" as DIGIT_4,
        46: "5" as DIGIT_5,
        47: "6" as DIGIT_6,
        48: "7" as DIGIT_7,
        49: "8" as DIGIT_8,
        50: "9" as DIGIT_9,
    },
    Extra {
        51: "" as EMPTY,
        52: "_" as WILDCARD,

        53: "C" as ABI_C,
        54: "Cool" as ABI_COOL,

        55: "main" as MAIN,
        56: "len" as LEN,
        57: "ptr" as PTR,

        58: "literal" as DIAG_LITERAL,
        59: "identifier" as DIAG_IDENT,
        60: "string literal" as DIAG_STR_LITERAL,
        61: "punctuation" as DIAG_PUNCTUATION,
        62: "binary operator" as DIAG_BIN_OP,
    },
}
