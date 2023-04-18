use crate::ItemPathBuf;
use cool_lexer::symbols::Symbol;
use thiserror::Error;

#[derive(Clone, Error, Debug)]
pub enum DefineError {
    #[error(transparent)]
    StructHasDuplicatedField(#[from] StructHasDuplicatedField),

    #[error(transparent)]
    StructHasInfiniteSize(#[from] StructHasInfiniteSize),

    #[error(transparent)]
    TypeCannotBeDefined(#[from] TypeCannotBeDefined),
}

#[derive(Clone, Error, Debug)]
#[error("struct {path} has a duplicated field \"{field}\"")]
pub struct StructHasDuplicatedField {
    pub path: ItemPathBuf,
    pub field: Symbol,
}

#[derive(Clone, Error, Debug)]
#[error("struct {path} has infinite size")]
pub struct StructHasInfiniteSize {
    pub path: ItemPathBuf,
}

#[derive(Clone, Error, Debug)]
#[error("type {path} cannot be defined")]
pub struct TypeCannotBeDefined {
    pub path: ItemPathBuf,
}
