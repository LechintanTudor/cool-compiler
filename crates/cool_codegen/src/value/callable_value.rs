use inkwell::values::{FunctionValue, PointerValue};

#[derive(Clone, Copy, Debug)]
pub enum CallableValue<'a> {
    Fn(FunctionValue<'a>),
    Register(PointerValue<'a>),
}

impl<'a> From<FunctionValue<'a>> for CallableValue<'a> {
    #[inline]
    fn from(value: FunctionValue<'a>) -> Self {
        Self::Fn(value)
    }
}
