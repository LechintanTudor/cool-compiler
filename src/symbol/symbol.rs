use crate::symbol::SYMBOL_TABLE;
use std::fmt;

pub type SymbolIndex = u32;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Symbol(pub SymbolIndex);

impl Symbol {
    pub fn is_keyword(&self) -> bool {
        self <= &sym::WHILE
    }

    pub fn is_bool_literal(&self) -> bool {
        self == &sym::FALSE || self == &sym::TRUE
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", SYMBOL_TABLE.get(*self))
    }
}

macro_rules! const_module {
    { $($kw:ident => ($idx:literal, $repr:literal),)+ } => {
        pub mod sym {
            use crate::symbol::{Symbol, SymbolTable};

            pub const ALL: &[&str] = &[$($repr,)+];

            $(
                pub const $kw: Symbol = Symbol($idx);
            )+

            pub(crate) fn intern_keywords(symbols: &mut SymbolTable) {
                $({
                    let symbol = symbols.insert($repr);
                    assert!(symbol.0 == $idx);
                })+
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

const_module! {
    BREAK => (0, "break"),
    CONTINUE => (1, "continue"),
    DEFER => (2, "defer"),
    ELSE => (3, "else"),
    ENUM => (4, "enum"),
    EXPORT => (5, "export"),
    FALSE => (6, "false"),
    FN => (7, "fn"),
    IF => (8, "if"),
    IMPORT => (9, "import"),
    MODULE => (10, "module"),
    MUT => (11, "mut"),
    STRUCT => (12, "struct"),
    TRUE => (13, "true"),
    WHILE => (14, "while"),
}
