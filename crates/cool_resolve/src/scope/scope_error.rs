use derive_more::{Display, Error};

pub type ScopeResult<T> = Result<T, ScopeError>;

#[derive(Clone, Copy, Error, Debug, Display)]
pub enum ScopeError {
    #[display("Symbol already exists")]
    SymbolAlreadyExists,
}
