use derive_more::Constructor;
use inkwell::types::BasicTypeEnum;
use inkwell::values::PointerValue;

#[derive(Clone, Copy, Constructor, Debug)]
pub struct MemoryValue<'a> {
    pub ptr: PointerValue<'a>,
    pub ty: BasicTypeEnum<'a>,
}
