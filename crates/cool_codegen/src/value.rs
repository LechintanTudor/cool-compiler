use crate::CodeGenerator;
use cool_resolve::TyId;
use inkwell::values::{BasicValue, BasicValueEnum, FunctionValue, PointerValue};

pub type LoadedValue<'a> = Option<BasicValueEnum<'a>>;

#[derive(Clone, Copy, Debug)]
pub enum Value<'a> {
    Void,
    Fn(FunctionValue<'a>),
    Register(BasicValueEnum<'a>),
    Memory(PointerValue<'a>),
}

impl<'a> Value<'a> {
    #[inline]
    pub fn is_void(&self) -> bool {
        matches!(self, Self::Void)
    }

    #[inline]
    pub fn into_function_value(self) -> FunctionValue<'a> {
        match self {
            Self::Fn(fn_value) => fn_value,
            _ => panic!("value is not a function"),
        }
    }
}

impl<'a> From<FunctionValue<'a>> for Value<'a> {
    #[inline]
    fn from(value: FunctionValue<'a>) -> Self {
        Self::Fn(value)
    }
}

impl<'a> From<LoadedValue<'a>> for Value<'a> {
    #[inline]
    fn from(value: LoadedValue<'a>) -> Self {
        match value {
            Some(value) => Self::Register(value),
            None => Self::Void,
        }
    }
}

impl<'a> From<BasicValueEnum<'a>> for Value<'a> {
    #[inline]
    fn from(value: BasicValueEnum<'a>) -> Self {
        Self::Register(value)
    }
}

impl<'a> CodeGenerator<'a> {
    pub fn gen_loaded_value(&self, ty_id: TyId, value: Value<'a>) -> LoadedValue<'a> {
        match value {
            Value::Void => LoadedValue::None,
            Value::Fn(fn_value) => fn_value.as_global_value().as_basic_value_enum().into(),
            Value::Memory(memory) => {
                let value_ty = self.tys[ty_id]?;
                self.builder.build_load(value_ty, memory, "").into()
            }
            Value::Register(value) => value.into(),
        }
    }
}
