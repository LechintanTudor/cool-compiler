mod function;
mod generated_tys;

use crate::generated_tys::GeneratedTys;
use cool_ast::PackageAst;
use cool_resolve::ResolveContext;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::{CodeModel, InitializationConfig, RelocMode, Target, TargetTriple};
use inkwell::OptimizationLevel;

pub struct CodeGenerator<'a> {
    context: &'a Context,
    target_triple: TargetTriple,
    resolve: &'a ResolveContext,
    tys: GeneratedTys<'a>,
    module: Module<'a>,
    builder: Builder<'a>,
}

impl<'a> CodeGenerator<'a> {
    pub fn create_context() -> Context {
        let context = Context::create();

        Target::initialize_all(&InitializationConfig {
            asm_parser: true,
            asm_printer: true,
            base: true,
            disassembler: true,
            info: true,
            machine_code: true,
        });

        context
    }

    pub fn new(context: &'a Context, target_triple: &str, resolve: &'a ResolveContext) -> Self {
        let target_triple = TargetTriple::create(target_triple);
        let target = Target::from_triple(&target_triple).unwrap();

        let module = context.create_module("Hello");
        module.set_source_file_name("main.cl");
        module.set_triple(&target_triple);

        let builder = context.create_builder();

        let target_machine = target
            .create_target_machine(
                &target_triple,
                "",
                "",
                OptimizationLevel::Default,
                RelocMode::Default,
                CodeModel::Default,
            )
            .unwrap();

        let target_data = target_machine.get_target_data();
        let tys = GeneratedTys::new(context, &target_data, resolve);

        Self {
            context,
            target_triple,
            resolve,
            tys,
            module,
            builder,
        }
    }

    pub fn gen_module(mut self, package: &PackageAst) -> Module<'a> {
        for extern_fn_ast in package.extern_fns.iter() {
            self.add_extern_fn(extern_fn_ast);
        }

        for fn_ast in package.fns.iter() {
            self.add_fn(fn_ast);
        }

        self.module
    }
}

// pub fn codegen() {
//     let context = Context::create();
//     let module = context.create_module("hello");
//     let builder = context.create_builder();

//     let i8_type = context.i8_type();

//     let i32_type = context.i32_type();
//     let i32_const_0 = i32_type.const_zero();

//     // Main function
//     let main_type = i32_type.fn_type(&[], false);
//     let main_item = module.add_function("main", main_type, None);
//     let main_body = context.append_basic_block(main_item, "entry");

//     // // Printf function
//     let printf_type = i32_type.fn_type(&[i8_type.ptr_type(AddressSpace::default()).into()], true);
//     let printf_item = module.add_function("printf", printf_type, None);

//     builder.position_at_end(main_body);
//     let format_str = builder.build_global_string_ptr("Hello, world!\n", "format_str");
//     builder.build_call(printf_item, &[format_str.as_pointer_value().into()], "");
//     builder.build_return(Some(&i32_const_0));

//     module.print_to_file("../programs/test.ll").unwrap();
// }
