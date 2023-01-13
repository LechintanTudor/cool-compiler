use crate::symbol::{kw, Symbol};
use bumpalo::Bump;
use rustc_hash::FxHashMap;

pub struct SymbolTable {
    bump: &'static Bump,
    symbols: FxHashMap<&'static str, Symbol>,
    strings: Vec<&'static str>,
}

impl Default for SymbolTable {
    fn default() -> Self {
        let mut symbols = Self {
            bump: Box::leak(Box::new(Bump::new())),
            symbols: Default::default(),
            strings: Default::default(),
        };

        kw::add_keywords(&mut symbols);
        symbols
    }
}

impl SymbolTable {
    pub fn insert(&mut self, symbol: &str) -> Symbol {
        if let Some(&symbol) = self.symbols.get(symbol) {
            return symbol;
        }

        let str = self.bump.alloc_str(symbol);
        let symbol = Symbol(self.strings.len() as u32);

        self.symbols.insert(str, symbol);
        self.strings.push(str);

        symbol
    }

    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.strings.iter().copied()
    }

    pub fn get(&self, symbol: Symbol) -> &str {
        self.strings[symbol.0 as usize]
    }
}
