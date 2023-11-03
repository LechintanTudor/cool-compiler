use cool_resolve::{ItemError, TyError};
use derive_more::{Display, Error, From};

pub type AstResult<T> = Result<T, AstError>;

#[derive(Clone, Error, From, Debug, Display)]
pub enum AstError {
    Item(ItemError),
    Ty(TyError),
}
