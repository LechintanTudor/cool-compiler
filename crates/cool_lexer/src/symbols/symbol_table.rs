use cool_arena::StrArena;
use cool_collections::id_newtype;
use std::fmt;

id_newtype!(Symbol; nodebug);

#[derive(Default)]
pub struct SymbolTable {
    symbols: StrArena<Symbol>,
}

impl SymbolTable {
    #[inline]
    pub(crate) fn insert_known(&mut self, expected_symbol: Symbol, symbol_str: &str) {
        let symbol = self.symbols.insert_if_not_exists(symbol_str).unwrap();

        assert_eq!(symbol, expected_symbol);
    }

    #[inline]
    pub fn insert(&mut self, symbol_str: &str) -> Symbol {
        self.symbols.get_or_insert(symbol_str)
    }

    #[inline]
    pub fn get(&self, symbol: Symbol) -> &str {
        &self.symbols[symbol]
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
