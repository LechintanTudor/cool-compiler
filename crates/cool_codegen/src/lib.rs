use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::AddressSpace;

pub struct Codegen<'a> {
    context: &'a Context,
    module: Module<'a>,
    builder: Builder<'a>,
}

// pub fn codegen(module_ast: ModuleAst, items: ItemTable, tys: TyTable) {
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
