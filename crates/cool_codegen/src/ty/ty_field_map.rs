use cool_lexer::Symbol;
use rustc_hash::FxHashMap;
use std::ops;

#[derive(Clone, Debug)]
pub struct TyFieldMap {
    fields: FxHashMap<Symbol, u32>,
}

impl TyFieldMap {
    #[inline]
    pub fn get(&self, symbol: Symbol) -> Option<u32> {
        self.fields.get(&symbol).copied()
    }
}

impl From<FxHashMap<Symbol, u32>> for TyFieldMap {
    #[inline]
    fn from(fields: FxHashMap<Symbol, u32>) -> Self {
        Self { fields }
    }
}

impl ops::Index<Symbol> for TyFieldMap {
    type Output = u32;

    #[inline]
    fn index(&self, symbol: Symbol) -> &Self::Output {
        &self.fields[&symbol]
    }
}
