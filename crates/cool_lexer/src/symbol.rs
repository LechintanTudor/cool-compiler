use crate::sym;
use cool_collections::{define_index_newtype, Arena};
use once_cell::sync::Lazy;
use std::fmt;
use std::sync::Mutex;

define_index_newtype!(Symbol; NoDebug);

pub(crate) type SymbolTable = Arena<Symbol, str>;

static SYMBOL_TABLE: Lazy<Mutex<SymbolTable>> = Lazy::new(|| {
    let mut symbols = SymbolTable::default();
    sym::insert_symbols(&mut symbols);
    Mutex::new(symbols)
});

impl Symbol {
    #[inline]
    pub fn insert(str: &str) -> Self {
        SYMBOL_TABLE.lock().unwrap().insert_str(str)
    }

    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        unsafe { &*(&SYMBOL_TABLE.lock().unwrap()[*self] as *const str) }
    }

    #[inline]
    #[must_use]
    pub fn is_keyword(&self) -> bool {
        &sym::kw_alias <= self && self <= &sym::kw_while
    }

    #[inline]
    #[must_use]
    pub fn is_path_keyword(&self) -> bool {
        [sym::kw_crate, sym::kw_super, sym::kw_self].contains(self)
    }

    #[inline]
    #[must_use]
    pub fn is_bool_literal(&self) -> bool {
        self == &sym::kw_true || self == &sym::kw_false
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
        write!(f, "{}", self.as_str())
    }
}
