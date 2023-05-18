use inkwell::types::BasicTypeEnum;
use inkwell::values::PointerValue;

#[derive(Clone, Copy, Debug)]
pub struct MemoryValue<'a> {
    pub pointer: PointerValue<'a>,
    pub ty: BasicTypeEnum<'a>,
}
