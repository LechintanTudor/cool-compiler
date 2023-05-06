mod expr;
mod function;
mod generated_tys;
mod stmt;
mod utils;
mod value;

pub use self::expr::*;
pub use self::function::*;
pub use self::generated_tys::*;
pub use self::stmt::*;
pub use self::utils::*;
pub use self::value::*;
use cool_ast::PackageAst;
use cool_resolve::{BindingId, ItemId, ResolveContext};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::PassManager;
use inkwell::targets::{CodeModel, InitializationConfig, RelocMode, Target, TargetTriple};
use inkwell::values::{FunctionValue, InstructionValue, IntValue};
use inkwell::OptimizationLevel;
use rustc_hash::FxHashMap;

pub struct CodeGenerator<'a> {
    context: &'a Context,
    llvm_true: IntValue<'a>,
    resolve: &'a ResolveContext,
    tys: GeneratedTys<'a>,
    fns: FxHashMap<ItemId, FunctionValue<'a>>,
    bindings: FxHashMap<BindingId, Value<'a>>,
    module: Module<'a>,
    pass_manager: PassManager<FunctionValue<'a>>,
    builder: Builder<'a>,
    fn_value: Option<FunctionValue<'a>>,
    last_alloca: Option<InstructionValue<'a>>,
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
        resolve: &'a ResolveContext,
        target_triple: &str,
        crate_name: &str,
        crate_root_file: &str,
    ) -> Self {
        let target_triple = TargetTriple::create(target_triple);
        let target = Target::from_triple(&target_triple).unwrap();

        let llvm_true = context.bool_type().const_int(1, false);

        let module = context.create_module(crate_name);
        module.set_source_file_name(crate_root_file);
        module.set_triple(&target_triple);

        let pass_manager = PassManager::create(&module);
        pass_manager.initialize();

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
            llvm_true,
            resolve,
            tys,
            fns: Default::default(),
            bindings: Default::default(),
            module,
            pass_manager,
            builder,
            fn_value: None,
            last_alloca: None,
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

        self.module.verify().unwrap();
        self.module
    }
}
