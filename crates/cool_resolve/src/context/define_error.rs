use crate::ItemPathBuf;
use cool_lexer::Symbol;
use derive_more::Error;
use std::fmt;

pub type DefineResult<T> = Result<T, DefineError>;

#[derive(Clone, Debug)]
pub enum DefineErrorKind {
    TypeCannotBeDefined,
    StructHasInfiniteSize,
    StructHasDuplicatedField { field: Symbol },
    EnumHasDuplicatedVariant { field: Symbol },
}

#[derive(Clone, Error, Debug)]
pub struct DefineError {
    pub path: ItemPathBuf,
    pub kind: DefineErrorKind,
}

impl fmt::Display for DefineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            DefineErrorKind::TypeCannotBeDefined => {
                write!(f, "type '{}' cannot be defined", self.path)
            }
            DefineErrorKind::StructHasInfiniteSize => {
                write!(f, "struct '{}' has infinite size", self.path)
            }
            DefineErrorKind::StructHasDuplicatedField { field } => {
                write!(f, "struct '{}' has duplicated field '{}'", self.path, field)
            }
            DefineErrorKind::EnumHasDuplicatedVariant { field: variant } => {
                write!(
                    f,
                    "enum '{}' has duplicated variant '{}'",
                    self.path, variant,
                )
            }
        }
    }
}
