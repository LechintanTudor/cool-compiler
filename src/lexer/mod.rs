mod identifier_table;
mod literal_table;
mod scanner;
mod tokens;

pub use self::identifier_table::*;
pub use self::literal_table::*;
pub use self::scanner::*;
pub use self::tokens::*;

pub fn tokenize(source: &[u8]) -> anyhow::Result<(Vec<Token>, IdentifierTable, LiteralTable)> {
    let mut scanner = Scanner::default();

    for &byte in source.iter() {
        scanner.consume(byte)?;
    }

    Ok(scanner.into_program())
}
