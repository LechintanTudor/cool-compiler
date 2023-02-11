use crate::symbols::SymbolTable;
use crate::tokens;
use lazy_static::lazy_static;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub type Lock<T> = RwLock<T>;
pub type ReadGuard<'a, T> = RwLockReadGuard<'a, T>;
pub type WriteGuard<'a, T> = RwLockWriteGuard<'a, T>;

lazy_static! {
    static ref SYMBOL_TABLE: Lock<SymbolTable> = {
        let mut symbols = SymbolTable::default();
        tokens::intern_symbols(&mut symbols);
        Lock::new(symbols)
    };
}

pub fn read_symbol_table() -> ReadGuard<'static, SymbolTable> {
    SYMBOL_TABLE.read().unwrap()
}

pub fn write_symbol_table() -> WriteGuard<'static, SymbolTable> {
    SYMBOL_TABLE.write().unwrap()
}
