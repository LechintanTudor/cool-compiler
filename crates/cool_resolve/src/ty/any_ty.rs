use crate::{ItemId, ValueTy};
use derive_more::From;

#[derive(Clone, PartialEq, Eq, Hash, From, Debug)]
pub enum AnyTy {
    Infer(InferTy),
    Value(ValueTy),
    Diverge,
    Item(ItemTy),
    StructDecl(ItemId),
}

impl AnyTy {
    #[inline]
    pub fn as_value(&self) -> Option<&ValueTy> {
        match self {
            Self::Value(value_ty) => Some(value_ty),
            _ => None,
        }
    }
}

impl Default for AnyTy {
    #[inline]
    fn default() -> Self {
        Self::Infer(InferTy::Any)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
pub enum InferTy {
    #[default]
    Any,
    Number,
    Int,
    Float,
    Subscript,
    EmptyArray,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ItemTy {
    Module,
    Ty,
}
