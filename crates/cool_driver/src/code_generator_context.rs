use inkwell::context::Context;
use inkwell::targets::{TargetData, TargetTriple};

pub struct CodeGeneratorContext {
    pub context: Context,
    pub target_triple: TargetTriple,
    pub target_data: TargetData,
}
