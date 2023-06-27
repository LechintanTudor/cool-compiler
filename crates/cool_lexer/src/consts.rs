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
                let mut insert_checked = |expected_id, symbol| {
                    let actual_id = symbols.insert_str(symbol);
                    assert_eq!(actual_id, expected_id);
                    actual_id
                };

                paste! {
                    // Keywords
                    $(
                        insert_checked([<KW_ $kw:upper>], stringify!($kw));
                    )+

                    // Primitives
                    $(
                        insert_checked([<$primitive:upper>], stringify!($primitive));
                    )+

                    // Digits
                    $(
                        insert_checked($digit, $digit_repr);
                    )+

                    // Extra
                    $(
                        insert_checked($extra, $extra_repr);
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
         1: align_of,
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
        13: for,
        14: if,
        15: loop,
        16: match,
        17: module,
        18: mut,
        19: offset_of,
        20: return,
        21: self,
        22: size_of,
        23: struct,
        24: super,
        25: switch,
        26: true,
        27: type,
        28: use,
        29: while,
    },
    Primitives {
        30: i8,
        31: i16,
        32: i32,
        33: i64,
        34: i128,
        35: isize,

        36: u8,
        37: u16,
        38: u32,
        39: u64,
        40: u128,
        41: usize,

        42: f32,
        43: f64,

        44: char,
        45: bool,
    },
    Digits {
        46: "0" as DIGIT_0,
        47: "1" as DIGIT_1,
        48: "2" as DIGIT_2,
        49: "3" as DIGIT_3,
        50: "4" as DIGIT_4,
        51: "5" as DIGIT_5,
        52: "6" as DIGIT_6,
        53: "7" as DIGIT_7,
        54: "8" as DIGIT_8,
        55: "9" as DIGIT_9,
    },
    Extra {
        56: "" as EMPTY,
        57: "_" as WILDCARD,

        58: "C" as ABI_C,
        59: "Cool" as ABI_COOL,

        60: "main" as MAIN,
        61: "len" as LEN,
        62: "ptr" as PTR,

        63: "literal" as DIAG_LITERAL,
        64: "identifier" as DIAG_IDENT,
        65: "string literal" as DIAG_STR_LITERAL,
        66: "punctuation" as DIAG_PUNCTUATION,
        67: "binary operator" as DIAG_BIN_OP,
    },
}
