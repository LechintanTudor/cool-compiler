use crate::item::ItemPathBuf;
use std::error::Error;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ImportErrorKind {
    SymbolNotFound,
    SymbolAlreadyImported,
    SymbolIsUnreachable,
}

#[derive(Clone, Debug)]
pub struct ImportError {
    pub kind: ImportErrorKind,
    pub module_path: ItemPathBuf,
    pub use_path: ItemPathBuf,
}

impl Error for ImportError {}

impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Failed to import '{}' in '{}': ",
            self.use_path, self.module_path
        )?;

        let symbol = self.use_path.last();

        match self.kind {
            ImportErrorKind::SymbolNotFound => write!(f, "symbol '{}' was not found", symbol)?,
            ImportErrorKind::SymbolAlreadyImported => {
                write!(f, "symbol '{}' was already imported", symbol)?
            }
            ImportErrorKind::SymbolIsUnreachable => {
                write!(f, "symbol '{}' is unreachable", symbol)?
            }
        }

        Ok(())
    }
}
