use crate::symbols::{InternedStr, Symbol};
use bumpalo::Bump;
use rustc_hash::FxHashMap;

pub struct SymbolTable {
    bump: Bump,
    symbols: FxHashMap<InternedStr, Symbol>,
    strings: Vec<InternedStr>,
}

unsafe impl Sync for SymbolTable {}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            bump: Default::default(),
            symbols: Default::default(),
            strings: vec![InternedStr::empty()],
        }
    }

    pub fn insert(&mut self, symbol_str: &str) -> Symbol {
        if let Some(&symbol) = self.symbols.get(symbol_str) {
            return symbol;
        }

        let symbol_str = unsafe {
            let symbol_str = self.bump.alloc_str(symbol_str);
            InternedStr::from_str(symbol_str)
        };
        let symbol = Symbol::new(self.strings.len() as u32).expect("ran out of symbol indexes");

        self.symbols.insert(symbol_str, symbol);
        self.strings.push(symbol_str);
        symbol
    }

    pub fn get(&self, symbol: Symbol) -> &str {
        self.strings[symbol.index() as usize].as_str()
    }

    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.strings[1..].iter().map(InternedStr::as_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbol_table() {
        let mut table = SymbolTable::empty();

        // The symbol table should start out empty.
        assert!(table.symbols.is_empty());
        assert!(table.strings.is_empty());

        // Add first element.
        let s1 = "s1";
        let sym1 = table.insert(s1);
        assert_eq!(table.symbols[s1], sym1);
        assert_eq!(table.strings.len(), 1);
        assert_eq!(table.strings[0].as_str(), s1);

        // Add a different second element.
        let s2 = "s2";
        let sym2 = table.insert(s2);
        assert_eq!(table.symbols[s2], sym2);
        assert_eq!(table.strings.len(), 2);
        assert_eq!(table.strings[1].as_str(), s2);

        // The symbols should be different.
        assert_ne!(s1, s2);
        assert_ne!(sym1, sym2);

        // Add the first element again.
        let s1_copy = "s1";
        let sym1_copy = table.insert(s1_copy);
        assert_eq!(table.symbols[s1_copy], sym1_copy);
        assert_eq!(table.strings.len(), 2);
        assert_eq!(table.strings[0].as_str(), s1_copy);

        // The first element copy should be equal to the original.
        assert_eq!(s1, s1_copy);
        assert_eq!(sym1, sym1_copy);
    }
}
