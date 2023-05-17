use crate::LoadedValue;
use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};

#[derive(Clone, Copy, Debug)]
pub enum Value<'a> {
    Void,
    Fn(FunctionValue<'a>),
    Memory {
        pointer: PointerValue<'a>,
        ty: BasicTypeEnum<'a>,
    },
    Register(BasicValueEnum<'a>),
}

impl<'a> Value<'a> {
    #[inline]
    pub fn into_function_value(self) -> FunctionValue<'a> {
        match self {
            Self::Fn(fn_value) => fn_value,
            _ => panic!("value is not a function"),
        }
    }
}

impl<'a> From<LoadedValue<'a>> for Value<'a> {
    #[inline]
    fn from(value: LoadedValue<'a>) -> Self {
        value.into_value()
    }
}

impl<'a> From<FunctionValue<'a>> for Value<'a> {
    #[inline]
    fn from(value: FunctionValue<'a>) -> Self {
        Self::Fn(value)
    }
}

impl<'a> From<BasicValueEnum<'a>> for Value<'a> {
    #[inline]
    fn from(value: BasicValueEnum<'a>) -> Self {
        Self::Register(value)
    }
}
