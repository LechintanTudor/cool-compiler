use crate::symbols::{sym, Symbol};
use cool_arena::StrArena;
use cool_collections::{Id, SmallString};
use once_cell::sync::Lazy;
use std::fmt::Write;
use std::sync::Mutex;

pub(crate) type SymbolTable<'a> = StrArena<'a, Symbol>;

static SYMBOL_TABLE: Lazy<Mutex<SymbolTable<'static>>> = Lazy::new(|| {
    let mut symbols = SymbolTable::new_leak();
    sym::intern_symbols(&mut symbols);
    Mutex::new(symbols)
});

impl Symbol {
    #[inline]
    pub fn insert(symbol_str: &str) -> Symbol {
        SYMBOL_TABLE.lock().unwrap().get_or_insert(symbol_str)
    }

    pub fn insert_u32(value: u32) -> Symbol {
        if value <= 9 {
            return sym::ALL_DIGITS[value as usize];
        }

        let mut value_str = SmallString::new();
        write!(&mut value_str, "{}", value).unwrap();
        Self::insert(&value_str)
    }

    #[inline]
    pub fn as_str(&self) -> &'static str {
        if *self <= sym::WILDCARD {
            sym::ALL_REPRS[self.index()]
        } else {
            SYMBOL_TABLE.lock().unwrap().get(*self).unwrap()
        }
    }
}
