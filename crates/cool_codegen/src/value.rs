use inkwell::types::BasicTypeEnum;
use inkwell::values::{AnyValueEnum, PointerValue};

#[derive(Clone, Copy, Debug)]
pub enum Value<'a> {
    Void,
    Lvalue {
        pointer: PointerValue<'a>,
        ty: BasicTypeEnum<'a>,
    },
    Rvalue(AnyValueEnum<'a>),
}

impl<'a> From<AnyValueEnum<'a>> for Value<'a> {
    #[inline]
    fn from(value: AnyValueEnum<'a>) -> Self {
        Self::Rvalue(value)
    }
}
