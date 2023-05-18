use crate::{CallableValue, Value};
use inkwell::values::BasicValueEnum;

#[derive(Clone, Copy, Debug)]
pub enum LoadedValue<'a> {
    Void,
    Register(BasicValueEnum<'a>),
}

impl<'a> LoadedValue<'a> {
    #[inline]
    pub fn into_value(self) -> Value<'a> {
        match self {
            Self::Void => Value::Void,
            Self::Register(value) => Value::Register(value),
        }
    }

    #[inline]
    pub fn into_callable_value(self) -> CallableValue<'a> {
        match self {
            Self::Register(value) => CallableValue::Register(value.into_pointer_value()),
            _ => panic!("loaded value cannot be converted to callable value"),
        }
    }

    #[inline]
    pub fn into_basic_value(self) -> BasicValueEnum<'a> {
        match self {
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

impl<'a> From<BasicValueEnum<'a>> for LoadedValue<'a> {
    #[inline]
    fn from(value: BasicValueEnum<'a>) -> Self {
        Self::Register(value)
    }
}
