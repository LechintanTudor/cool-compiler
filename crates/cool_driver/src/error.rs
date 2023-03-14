use cool_lexer::tokens::TokenKind;
use cool_resolve::item::ItemError;
use std::error::Error;
use std::fmt;
use std::path::PathBuf;

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

#[derive(Clone, Debug)]
pub struct ParserError {
    pub path: PathBuf,
    pub line: u32,
    pub found: TokenKind,
    pub expected: &'static [TokenKind],
}

impl Error for ParserError {}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Failed to parse file: \"{}\"", self.path.display())?;
        writeln!(f, " -> Error on line {}", self.line)?;
        writeln!(f, " -> Expected one of {:?}", self.expected)?;
        writeln!(f, " -> Found: {}", self.found)
    }
}
