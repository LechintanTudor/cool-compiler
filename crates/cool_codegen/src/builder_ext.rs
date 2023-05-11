use inkwell::builder::Builder;
use inkwell::values::IntValue;
use inkwell::IntPredicate;

pub trait BuilderExt<'a> {
    fn build_bool(&self, value: IntValue<'a>, name: &str) -> IntValue<'a>;
}

impl<'a> BuilderExt<'a> for Builder<'a> {
    fn build_bool(&self, value: IntValue<'a>, name: &str) -> IntValue<'a> {
        self.build_int_compare(
            IntPredicate::EQ,
            value,
            value.get_type().const_int(1, false),
            name,
        )
    }
}
