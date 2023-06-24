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
        15: match,
        16: module,
        17: mut,
        18: return,
        19: self,
        20: struct,
        21: super,
        22: switch,
        23: true,
        24: type,
        25: use,
        26: while,
    },
    Primitives {
        27: i8,
        28: i16,
        29: i32,
        30: i64,
        31: i128,
        32: isize,

        33: u8,
        34: u16,
        35: u32,
        36: u64,
        37: u128,
        38: usize,

        39: f32,
        40: f64,

        41: char,
        42: bool,
    },
    Digits {
        43: "0" as DIGIT_0,
        44: "1" as DIGIT_1,
        45: "2" as DIGIT_2,
        46: "3" as DIGIT_3,
        47: "4" as DIGIT_4,
        48: "5" as DIGIT_5,
        49: "6" as DIGIT_6,
        50: "7" as DIGIT_7,
        51: "8" as DIGIT_8,
        52: "9" as DIGIT_9,
    },
    Extra {
        53: "" as EMPTY,
        54: "_" as WILDCARD,

        55: "C" as ABI_C,
        56: "Cool" as ABI_COOL,

        57: "main" as MAIN,
        58: "len" as LEN,
        59: "ptr" as PTR,

        60: "literal" as DIAG_LITERAL,
        61: "identifier" as DIAG_IDENT,
        62: "string literal" as DIAG_STR_LITERAL,
        63: "punctuation" as DIAG_PUNCTUATION,
        64: "binary operator" as DIAG_BIN_OP,
    },
}
