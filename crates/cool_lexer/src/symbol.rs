use crate::sym;
use cool_arena::{define_arena_index, Arena};
use once_cell::sync::Lazy;
use std::fmt;
use std::num::NonZeroU32;
use std::sync::Mutex;

define_arena_index!(Symbol; NoDebug);

pub(crate) type SymbolTable<'a> = Arena<'a, Symbol, str>;

static SYMBOL_TABLE: Lazy<Mutex<SymbolTable<'static>>> = Lazy::new(|| {
    let mut symbols = SymbolTable::new_leak();
    sym::insert_symbols(&mut symbols);
    Mutex::new(symbols)
});

impl Symbol {
    #[inline]
    pub(crate) const unsafe fn new_unchecked(index: u32) -> Self {
        Self(NonZeroU32::new_unchecked(index))
    }

    #[inline]
    pub fn insert(str: &str) -> Self {
        SYMBOL_TABLE.lock().unwrap().insert_str(str)
    }

    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        SYMBOL_TABLE.lock().unwrap().get(*self).unwrap()
    }

    #[inline]
    #[must_use]
    pub fn is_keyword(&self) -> bool {
        &sym::kw_alias <= self && self <= &sym::kw_while
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
