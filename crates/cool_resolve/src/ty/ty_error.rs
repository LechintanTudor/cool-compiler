use cool_lexer::Symbol;
use derive_more::{Display, Error};

pub type TyResult<T> = Result<T, TyError>;

#[derive(Clone, Error, Debug, Display)]
pub enum TyError {
    #[display("Type cannot be define")]
    CannotBeDefined,

    #[display("Function has an unknown abi")]
    UnknownAbi { abi: Symbol },
}
