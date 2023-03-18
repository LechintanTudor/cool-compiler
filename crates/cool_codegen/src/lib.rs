mod ty_codegen;

use crate::ty_codegen::GeneratedTys;
use cool_ast::{ItemAst, ModuleItemAst};
use cool_resolve::item::ItemTable;
use cool_resolve::ty::TyTable;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::{CodeModel, InitializationConfig, RelocMode, Target, TargetTriple};
use inkwell::{AddressSpace, OptimizationLevel};

pub struct Codegen<'ctx> {
    context: &'ctx Context,
    target_triple: TargetTriple,
    tys: GeneratedTys<'ctx>,
    item_table: &'ctx ItemTable,
}

impl<'ctx> Codegen<'ctx> {
    pub fn new(
        context: &'ctx Context,
        target_triple: &str,
        item_table: &'ctx ItemTable,
        ty_table: &TyTable,
    ) -> Self {
        let target_triple = TargetTriple::create(target_triple);
        let target = Target::from_triple(&target_triple).unwrap();

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
        let tys = GeneratedTys::new(context, &target_data, ty_table);

        Self {
            context,
            target_triple,
            tys,
            item_table,
        }
    }

    pub fn run_for_module(&self, module_ast: &ModuleItemAst) -> Module<'ctx> {
        let module = self.context.create_module("todo");
        module.set_source_file_name("todo.cl");
        module.set_triple(&self.target_triple);

        for decl in module_ast.decls.iter() {
            match &decl.item {
                ItemAst::Fn(fn_ast) => {
                    let fn_type = self.tys.get(fn_ast.ty_id).into_function_type();
                    let fn_item = module.add_function(decl.symbol.as_str(), fn_type, None);
                    let _fn_body = self.context.append_basic_block(fn_item, "entry");
                }
                _ => todo!(),
            }
        }

        module
    }
}

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

pub fn codegen() {
    let context = Context::create();
    let module = context.create_module("hello");
    let builder = context.create_builder();

    let i8_type = context.i8_type();

    let i32_type = context.i32_type();
    let i32_const_0 = i32_type.const_zero();

    // Main function
    let main_type = i32_type.fn_type(&[], false);
    let main_item = module.add_function("main", main_type, None);
    let main_body = context.append_basic_block(main_item, "entry");

    // // Printf function
    let printf_type = i32_type.fn_type(&[i8_type.ptr_type(AddressSpace::default()).into()], true);
    let printf_item = module.add_function("printf", printf_type, None);

    builder.position_at_end(main_body);
    let format_str = builder.build_global_string_ptr("Hello, world!\n", "format_str");
    builder.build_call(printf_item, &[format_str.as_pointer_value().into()], "");
    builder.build_return(Some(&i32_const_0));

    module.print_to_file("../programs/test.ll").unwrap();
}
