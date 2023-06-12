use crate::{InferTy, ItemTy, ValueTy};
use derive_more::From;

#[derive(Clone, Eq, PartialEq, Hash, From, Debug)]
pub enum AnyTy {
    Infer(InferTy),
    Item(ItemTy),
    Value(ValueTy),
}
