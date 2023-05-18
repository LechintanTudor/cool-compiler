use crate::{LoadedValue, MemoryValue};
use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};

#[derive(Clone, Copy, Debug)]
pub enum Value<'a> {
    Void,
    Fn(FunctionValue<'a>),
    Memory(MemoryValue<'a>),
    Register(BasicValueEnum<'a>),
}

impl<'a> Value<'a> {
    #[inline]
    pub fn memory(pointer: PointerValue<'a>, ty: BasicTypeEnum<'a>) -> Value<'a> {
        Self::Memory(MemoryValue { pointer, ty })
    }

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

impl<'a> From<MemoryValue<'a>> for Value<'a> {
    #[inline]
    fn from(value: MemoryValue<'a>) -> Self {
        Self::Memory(value)
    }
}

impl<'a> From<BasicValueEnum<'a>> for Value<'a> {
    #[inline]
    fn from(value: BasicValueEnum<'a>) -> Self {
        Self::Register(value)
    }
}
