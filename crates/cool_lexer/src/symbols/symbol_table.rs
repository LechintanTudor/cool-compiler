use cool_arena::{handle_newtype, StrArena};
use std::fmt;

handle_newtype!(Symbol);

#[derive(Default)]
pub struct SymbolTable {
    symbols: StrArena,
}

impl SymbolTable {
    #[inline]
    pub fn insert(&mut self, symbol_str: &str) -> Symbol {
        Symbol(self.symbols.insert(symbol_str))
    }

    #[inline]
    pub fn get(&self, symbol: Symbol) -> &str {
        &self.symbols[symbol.0]
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.symbols.iter()
    }
}

impl fmt::Debug for SymbolTable {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.symbols, f)
    }
}
