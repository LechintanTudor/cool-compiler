use crate::symbols::sym;
use cool_arena::StrArena;
use cool_collections::{id_newtype, Id, SmallString};
use once_cell::sync::Lazy;
use std::fmt;
use std::fmt::Write;
use std::sync::Mutex;

id_newtype!(Symbol);

pub(crate) type SymbolTable<'a> = StrArena<'a, Symbol>;

static SYMBOL_TABLE: Lazy<Mutex<SymbolTable<'static>>> = Lazy::new(|| {
    let mut symbols = SymbolTable::new_leak();
    sym::intern_symbols(&mut symbols);
    Mutex::new(symbols)
});

impl Symbol {
    #[inline]
    pub fn insert(symbol_str: &str) -> Symbol {
        SYMBOL_TABLE.lock().unwrap().get_or_insert(symbol_str)
    }

    pub fn insert_u32(value: u32) -> Symbol {
        if value <= 9 {
            return sym::ALL_DIGITS[value as usize];
        }

        let mut value_str = SmallString::new();
        write!(&mut value_str, "{}", value).unwrap();
        Self::insert(&value_str)
    }

    #[inline]
    pub fn is_keyword(&self) -> bool {
        *self <= sym::KW_WHILE
    }

    #[inline]
    pub fn is_bool_literal(&self) -> bool {
        *self == sym::KW_FALSE || *self == sym::KW_TRUE
    }

    #[inline]
    pub fn is_known_suffix(&self) -> bool {
        *self >= sym::I8 && *self <= sym::F64
    }

    #[inline]
    pub fn as_str(&self) -> &'static str {
        if *self <= sym::WILDCARD {
            sym::ALL_REPRS[self.index()]
        } else {
            SYMBOL_TABLE.lock().unwrap().get(*self).unwrap()
        }
    }
}

impl fmt::Display for Symbol {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
