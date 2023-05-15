use crate::ItemPathBuf;
use cool_lexer::symbols::Symbol;
use derive_more::{Display, Error, From};

#[derive(Clone, Error, From, Display, Debug)]
pub enum DefineError {
    StructHasDuplicatedField(StructHasDuplicatedField),
    StructHasInfiniteSize(StructHasInfiniteSize),
    TyCannotBeDefined(TyCannotBeDefined),
}

#[derive(Clone, Error, Display, Debug)]
#[display(fmt = "struct {path} has duplicate field '{field}'")]
pub struct StructHasDuplicatedField {
    pub path: ItemPathBuf,
    pub field: Symbol,
}

#[derive(Clone, Error, Display, Debug)]
pub struct StructHasInfiniteSize {
    pub path: ItemPathBuf,
}

#[derive(Clone, Error, Display, Debug)]
pub struct TyCannotBeDefined {
    pub path: ItemPathBuf,
}
