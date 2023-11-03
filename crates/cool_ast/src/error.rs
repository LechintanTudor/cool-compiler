use cool_resolve::ResolveError;
use derive_more::{Display, Error, From};

pub type AstResult<T> = Result<T, AstError>;

#[derive(Clone, Error, From, Debug, Display)]
pub enum AstError {
    Resolve(ResolveError),
}
