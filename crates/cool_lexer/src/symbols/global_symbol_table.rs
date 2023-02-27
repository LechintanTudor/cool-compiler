use crate::consts::sym;
use crate::symbols::{Symbol, SymbolTable};
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref SYMBOL_TABLE: Mutex<SymbolTable> = {
        let mut symbols = SymbolTable::default();
        sym::intern_symbols(&mut symbols);
        Mutex::new(symbols)
    };
}

impl Symbol {
    pub fn insert(symbol_str: &str) -> Self {
        SYMBOL_TABLE.lock().unwrap().insert(symbol_str)
    }

    pub fn as_str_from_symbol_table(symbol: Self) -> &'static str {
        let symbols = SYMBOL_TABLE.lock().unwrap();
        let str = symbols.get(symbol);

        unsafe { std::mem::transmute::<_, &'static str>(str) }
    }
}
