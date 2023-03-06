use cool_resolve::item::ItemError;
use std::error::Error;
use std::fmt;

#[derive(Clone, Debug)]
pub struct CompileError {
    pub import_errors: Vec<ItemError>,
}

impl Error for CompileError {}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to compile crate.\n\n")?;

        if !self.import_errors.is_empty() {
            writeln!(f, "Import errors:")?;
            for error in self.import_errors.iter() {
                writeln!(f, "  - {:?} in {:?}", error.symbol_path, error.module_path)?;
            }
        }

        Ok(())
    }
}
