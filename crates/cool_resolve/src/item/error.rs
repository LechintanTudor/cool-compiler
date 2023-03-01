use crate::item::ItemPathBuf;
use std::error::Error;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ItemErrorKind {
    SymbolNotFound,
    SymbolAlreadyDefined,
    SymbolIsUnreachable,
    SymbolIsNotModule,
}

#[derive(Clone, Debug)]
pub struct ItemError {
    pub kind: ItemErrorKind,
    pub module_path: ItemPathBuf,
    pub symbol_path: ItemPathBuf,
}

impl Error for ItemError {}

impl fmt::Display for ItemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Failed to import '{}' in '{}': ",
            self.symbol_path, self.module_path,
        )?;

        let symbol = self.symbol_path.last();

        match self.kind {
            ItemErrorKind::SymbolNotFound => write!(f, "symbol '{}' was not found", symbol)?,
            ItemErrorKind::SymbolAlreadyDefined => {
                write!(f, "symbol '{}' was already defined", symbol)?
            }
            ItemErrorKind::SymbolIsUnreachable => write!(f, "symbol '{}' is unreachable", symbol)?,
            ItemErrorKind::SymbolIsNotModule => write!(f, "symbol '{}' is not a module", symbol)?,
        }

        Ok(())
    }
}
