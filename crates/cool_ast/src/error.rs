use cool_resolve::ResolveError;
use derive_more::{Display, Error, From};

pub type AstResult<T> = Result<T, AstError>;
pub type LiteralResult<T> = Result<T, LiteralError>;

#[derive(Clone, Error, From, Debug, Display)]
pub enum AstError {
    Resolve(ResolveError),
    Literal(LiteralError),
}

#[derive(Clone, Error, Debug, Display)]
pub enum LiteralError {
    ValueTooLarge,
    SuffixUnknown,
}
