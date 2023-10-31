use crate::ModulePathsError;
use cool_parser::ParseError;
use cool_resolve::ItemError;
use derive_more::{Display, Error, From};
use std::io::Error as IoError;

pub type CompileResult<T> = Result<T, CompileError>;

#[derive(From, Error, Debug, Display)]
pub enum CompileError {
    Path(ModulePathsError),
    Io(IoError),
    Parse(ParseError),
    Item(ItemError),
}
