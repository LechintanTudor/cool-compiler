mod builder_ext;
mod expr;
mod function;
mod stmt;
mod ty;
mod utils;
mod value;

pub use self::builder_ext::*;
pub use self::expr::*;
pub use self::function::*;
pub use self::stmt::*;
pub use self::ty::*;
pub use self::utils::*;
pub use self::value::*;
use cool_ast::PackageAst;
use cool_resolve::{BindingId, ResolveContext};
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::PassManager;
use inkwell::targets::{InitializationConfig, Target, TargetData, TargetTriple};
use inkwell::values::{FunctionValue, InstructionValue, IntValue};
use rustc_hash::FxHashMap;

#[derive(Clone, Debug)]
struct FnState<'a> {
    fn_value: FunctionValue<'a>,
    last_alloca: Option<InstructionValue<'a>>,
    ret_values: Vec<(LoadedValue<'a>, BasicBlock<'a>)>,
}

impl<'a> FnState<'a> {
    #[inline]
    pub fn new(fn_value: FunctionValue<'a>) -> Self {
        Self {
            fn_value,
            last_alloca: None,
            ret_values: Default::default(),
        }
    }
}

pub struct CodeGenerator<'a> {
    context: &'a Context,
    llvm_true: IntValue<'a>,
    llvm_false: IntValue<'a>,
    resolve: &'a ResolveContext,
    tys: GeneratedTys<'a>,
    bindings: FxHashMap<BindingId, Value<'a>>,
    module: Module<'a>,
    pass_manager: PassManager<FunctionValue<'a>>,
    builder: Builder<'a>,
    fn_stack: Vec<FnState<'a>>,
}

impl<'a> CodeGenerator<'a> {
    pub fn create_context() -> Context {
        let context = Context::create();

        Target::initialize_all(&InitializationConfig {
            asm_parser: false,
            asm_printer: false,
            base: true,
            disassembler: false,
            info: true,
            machine_code: true,
        });

        context
    }

    pub fn new(
        context: &'a Context,
        target_triple: &TargetTriple,
        target_data: &TargetData,
        resolve: &'a ResolveContext,
        crate_name: &str,
        crate_root_file: &str,
    ) -> Self {
        let llvm_true = context.i8_type().const_int(1, false);
        let llvm_false = context.i8_type().const_int(0, false);

        let module = context.create_module(crate_name);
        module.set_source_file_name(crate_root_file);
        module.set_triple(target_triple);

        let pass_manager = PassManager::create(&module);
        //pass_manager.add_promote_memory_to_register_pass();
        pass_manager.initialize();

        let builder = context.create_builder();
        let tys = GeneratedTys::new(context, target_data, resolve);

        Self {
            context,
            llvm_true,
            llvm_false,
            resolve,
            tys,
            bindings: Default::default(),
            module,
            pass_manager,
            builder,
            fn_stack: Default::default(),
        }
    }

    pub fn gen_module(mut self, package: &PackageAst) -> Module<'a> {
        for extern_fn_ast in package.extern_fns.iter() {
            self.add_extern_fn(extern_fn_ast);
        }

        for fn_ast in package.fns.iter() {
            self.add_fn(fn_ast);
        }

        for fn_ast in package.fns.iter() {
            self.gen_fn(fn_ast);
        }

        self.module
    }

    pub fn get_current_entry_block(&self) -> BasicBlock<'a> {
        self.fn_stack
            .last()
            .unwrap()
            .fn_value
            .get_first_basic_block()
            .unwrap()
    }

    pub fn get_last_alloca(&self) -> Option<InstructionValue<'a>> {
        self.fn_stack.last().unwrap().last_alloca
    }

    pub fn update_last_alloca(&mut self, alloca: InstructionValue<'a>) {
        self.fn_stack.last_mut().unwrap().last_alloca = Some(alloca);
    }

    pub fn add_ret_value(&mut self, value: LoadedValue<'a>, block: BasicBlock<'a>) {
        self.fn_stack
            .last_mut()
            .unwrap()
            .ret_values
            .push((value, block))
    }
}
