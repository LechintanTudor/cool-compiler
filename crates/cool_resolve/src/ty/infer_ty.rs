use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum InferTy {
    Any,
    Int,
    Float,
    Number,
    EmptyArray,
}

impl fmt::Display for InferTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_str = match self {
            Self::Any => "<any>",
            Self::Int => "<int>",
            Self::Float => "<float>",
            Self::Number => "<number>",
            Self::EmptyArray => "<array>",
        };

        write!(f, "{display_str}")
    }
}
