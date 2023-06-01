mod symbol_table;

pub(crate) use self::symbol_table::*;
pub use crate::consts::sym;
use cool_collections::id_newtype;
use std::fmt;

id_newtype!(Symbol; no_debug);

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
}

impl fmt::Debug for Symbol {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.as_str())
    }
}

impl fmt::Display for Symbol {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
