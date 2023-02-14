use crate::symbols::Symbol;
use cool_arena::StrArena;

#[derive(Default, Debug)]
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
        self.symbols.get(symbol.0)
    }

    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.symbols.iter()
    }
}
