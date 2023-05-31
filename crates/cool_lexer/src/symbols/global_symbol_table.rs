use crate::consts::sym;
use crate::symbols::{Symbol, SymbolTable};
use cool_collections::SmallString;
use once_cell::sync::Lazy;
use std::fmt::Write;
use std::sync::Mutex;

static SYMBOL_TABLE: Lazy<Mutex<SymbolTable<'static>>> = Lazy::new(|| {
    let bump = Box::leak(Box::default());
    let mut symbols = unsafe { SymbolTable::new(bump) };
    sym::intern_symbols(&mut symbols);
    Mutex::new(symbols)
});

impl Symbol {
    #[inline]
    pub fn insert(symbol_str: &str) -> Self {
        SYMBOL_TABLE.lock().unwrap().insert(symbol_str)
    }

    pub fn insert_u32(n: u32) -> Self {
        let mut buffer = SmallString::new();
        write!(&mut buffer, "{n}").unwrap();
        Self::insert(&buffer)
    }

    #[inline]
    pub fn as_str_from_symbol_table(symbol: Self) -> &'static str {
        SYMBOL_TABLE.lock().unwrap().get(symbol)
    }
}
