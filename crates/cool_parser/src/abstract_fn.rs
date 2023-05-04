use crate::{ExternFnItem, FnExpr};
use derive_more::From;

#[derive(Clone, From, Debug)]
pub enum AbstractFn {
    Fn(FnExpr),
    ExternFn(ExternFnItem),
}
