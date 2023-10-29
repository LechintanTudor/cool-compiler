pub type TyResult<T> = Result<T, TyError>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TyError {
    CannotBeDefined,
}
