use crate::{CodeGeneratorContext, CompileError, CompileResult, InitError};
use cool_resolve::{PrimitiveTyData, ResolveContext};
use inkwell::context::Context;
use inkwell::targets::{CodeModel, InitializationConfig, RelocMode, Target, TargetTriple};
use inkwell::OptimizationLevel;

pub fn p0_init(target_triple: &str) -> CompileResult<(CodeGeneratorContext, ResolveContext)> {
    let context = Context::create();

    Target::initialize_all(&InitializationConfig {
        asm_parser: false,
        asm_printer: false,
        base: true,
        disassembler: false,
        info: true,
        machine_code: true,
    });

    let target_triple = TargetTriple::create(target_triple);
    let target = Target::from_triple(&target_triple).map_err(|message| {
        CompileError::from_kind(InitError {
            message: message.to_string(),
        })
    })?;

    let target_machine = target
        .create_target_machine(
            &target_triple,
            "",
            "",
            OptimizationLevel::Default,
            RelocMode::Default,
            CodeModel::Default,
        )
        .ok_or_else(|| {
            CompileError::from_kind(InitError {
                message: "failed to create target machine".to_owned(),
            })
        })?;

    let target_data = target_machine.get_target_data();
    let ptr_type = context.ptr_sized_int_type(&target_data, None);

    let primitives = PrimitiveTyData {
        i8_align: target_data.get_preferred_alignment(&context.i8_type()) as _,
        i16_align: target_data.get_preferred_alignment(&context.i16_type()) as _,
        i32_align: target_data.get_preferred_alignment(&context.i32_type()) as _,
        i64_align: target_data.get_preferred_alignment(&context.i64_type()) as _,
        i128_align: target_data.get_preferred_alignment(&context.i128_type()) as _,
        f32_align: target_data.get_preferred_alignment(&context.f32_type()) as _,
        f64_align: target_data.get_preferred_alignment(&context.f64_type()) as _,
        ptr_size: target_data.get_store_size(&ptr_type),
        ptr_align: target_data.get_preferred_alignment(&ptr_type) as _,
    };

    Ok((
        CodeGeneratorContext {
            context,
            target_triple,
            target_data,
        },
        ResolveContext::new(primitives),
    ))
}
