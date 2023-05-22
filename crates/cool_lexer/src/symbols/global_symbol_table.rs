use crate::consts::sym;
use crate::symbols::{Symbol, SymbolTable};
use once_cell::sync::Lazy;
use std::sync::Mutex;

static SYMBOL_TABLE: Lazy<Mutex<SymbolTable>> = Lazy::new(|| {
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

    pub fn as_str_from_symbol_table(symbol: Self) -> &'static str {
        let symbols = SYMBOL_TABLE.lock().unwrap();
        let str = symbols.get(symbol);

        unsafe { std::mem::transmute::<_, &'static str>(str) }
    }
}
