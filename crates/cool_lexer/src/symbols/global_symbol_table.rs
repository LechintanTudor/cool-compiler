use crate::symbols::{Symbol, SymbolTable};
use crate::tokens;
use lazy_static::lazy_static;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

lazy_static! {
    pub static ref SYMBOL_TABLE: GlobalSymbolTable = {
        let mut symbols = SymbolTable::new();
        tokens::intern_symbols(&mut symbols);
        GlobalSymbolTable(Lock::new(symbols))
    };
}

pub type Lock<T> = RwLock<T>;
pub type ReadLockGuard<'a, T> = RwLockReadGuard<'a, T>;
pub type WriteLockGuard<'a, T> = RwLockWriteGuard<'a, T>;

pub struct GlobalSymbolTable(Lock<SymbolTable>);

impl GlobalSymbolTable {
    pub fn read_inner(&self) -> ReadLockGuard<SymbolTable> {
        self.0.read().unwrap()
    }

    pub fn write_inner(&self) -> WriteLockGuard<SymbolTable> {
        self.0.write().unwrap()
    }

    pub fn insert(&self, symbol_str: &str) -> Symbol {
        let mut table = self.0.write().unwrap();
        table.insert(symbol_str)
    }

    pub fn get(&self, symbol: Symbol) -> &str {
        let table = self.0.read().unwrap();
        unsafe { &*(table.get(symbol) as *const str) }
    }
}
