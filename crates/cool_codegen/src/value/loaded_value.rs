use crate::{CallableValue, Value};
use inkwell::values::{BasicValue, BasicValueEnum, FunctionValue};

#[derive(Clone, Copy, Debug)]
pub enum LoadedValue<'a> {
    Void,
    Fn(FunctionValue<'a>),
    Register(BasicValueEnum<'a>),
}

impl<'a> LoadedValue<'a> {
    #[inline]
    pub fn into_value(self) -> Value<'a> {
        match self {
            Self::Void => Value::Void,
            Self::Fn(fn_value) => Value::Fn(fn_value),
            Self::Register(value) => Value::Register(value),
        }
    }

    #[inline]
    pub fn into_callable_value(self) -> CallableValue<'a> {
        match self {
            Self::Fn(fn_value) => CallableValue::Fn(fn_value),
            Self::Register(value) => CallableValue::Register(value.into_pointer_value()),
            _ => panic!("loaded value cannot be converted to callable value"),
        }
    }

    #[inline]
    pub fn into_basic_value(self) -> BasicValueEnum<'a> {
        match self {
            Self::Fn(fn_value) => {
                fn_value
                    .as_global_value()
                    .as_pointer_value()
                    .as_basic_value_enum()
            }
            Self::Register(value) => value,
            _ => panic!("loaded value is not a basic value"),
        }
    }

    #[inline]
    pub fn as_basic_value(&self) -> Option<&BasicValueEnum<'a>> {
        match self {
            Self::Register(value) => Some(value),
            _ => None,
        }
    }
}

impl<'a> From<FunctionValue<'a>> for LoadedValue<'a> {
    #[inline]
    fn from(value: FunctionValue<'a>) -> Self {
        Self::Fn(value)
    }
}

impl<'a> From<BasicValueEnum<'a>> for LoadedValue<'a> {
    #[inline]
    fn from(value: BasicValueEnum<'a>) -> Self {
        Self::Register(value)
    }
}
