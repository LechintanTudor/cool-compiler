#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum InferTy {
    Any,
    Int,
    Float,
    Number,
}
