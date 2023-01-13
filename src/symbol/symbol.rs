use std::fmt;

pub type SymbolIndex = u32;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Symbol(pub SymbolIndex);

impl Symbol {
    pub fn is_keyword(&self) -> bool {
        self.0 <= kw::WHILE
    }

    pub fn is_bool_literal(&self) -> bool {
        self.0 == kw::FALSE || self.0 == kw::TRUE
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_keyword() {
            write!(f, "{}", kw::ALL[self.0 as usize])
        } else {
            write!(f, "<ident {}>", self.0)
        }
    }
}

macro_rules! kw_module {
    { $($kw:ident => ($idx:literal, $repr:literal),)+ } => {
        pub mod kw {
            use crate::symbol::{SymbolIndex, SymbolTable};

            pub const ALL: &[&str] = &[$($repr,)+];

            $(
                pub const $kw: SymbolIndex = $idx;
            )+

            pub(crate) fn add_keywords(symbols: &mut SymbolTable) {
                $({
                    let symbol = symbols.insert($repr);
                    assert!(symbol.0 == $idx);
                })+
            }
        }
    };
}

kw_module! {
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
