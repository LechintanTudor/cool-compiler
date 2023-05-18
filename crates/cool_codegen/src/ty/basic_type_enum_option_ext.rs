use inkwell::types::{ArrayType, BasicTypeEnum, FloatType, IntType};

pub trait BaiscTypeEnumOptionExt<'a> {
    fn into_int_type(self) -> IntType<'a>;

    fn into_float_type(self) -> FloatType<'a>;

    fn into_array_type(self) -> ArrayType<'a>;
}

impl<'a> BaiscTypeEnumOptionExt<'a> for Option<BasicTypeEnum<'a>> {
    fn into_int_type(self) -> IntType<'a> {
        self.unwrap().into_int_type()
    }

    fn into_float_type(self) -> FloatType<'a> {
        self.unwrap().into_float_type()
    }

    fn into_array_type(self) -> ArrayType<'a> {
        self.unwrap().into_array_type()
    }
}
