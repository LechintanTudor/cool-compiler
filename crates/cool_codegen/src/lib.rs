mod builder_ext;
mod expr;
mod fn_state;
mod function;
mod stmt;
mod ty;
mod utils;
mod value;

pub use self::builder_ext::*;
pub use self::expr::*;
pub use self::fn_state::*;
pub use self::function::*;
pub use self::stmt::*;
pub use self::ty::*;
pub use self::utils::*;
pub use self::value::*;
use cool_ast::PackageAst;
use cool_resolve::{BindingId, FrameId, ResolveContext};
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::PassManager;
use inkwell::targets::{InitializationConfig, Target, TargetData, TargetTriple};
use inkwell::values::{FunctionValue, InstructionValue, IntValue};
use rustc_hash::{FxHashMap, FxHashSet};

pub struct CodeGenerator<'a> {
    context: &'a Context,
    package: &'a PackageAst,
    resolve: &'a ResolveContext,
    llvm_true: IntValue<'a>,
    llvm_false: IntValue<'a>,
    tys: GeneratedTys<'a>,
    bindings: FxHashMap<BindingId, Value<'a>>,
    module: Module<'a>,
    pass_manager: PassManager<FunctionValue<'a>>,
    builder: Builder<'a>,
    fn_stack: Vec<FnState<'a>>,
    visited_defers: FxHashSet<FrameId>,
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
        package: &'a PackageAst,
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
        //pass_manager.add_instruction_combining_pass();
        //pass_manager.add_reassociate_pass();
        //pass_manager.add_gvn_pass();
        //pass_manager.add_cfg_simplification_pass();
        pass_manager.initialize();

        let builder = context.create_builder();
        let tys = GeneratedTys::new(context, target_data, resolve);

        Self {
            context,
            package,
            resolve,
            llvm_true,
            llvm_false,
            tys,
            bindings: Default::default(),
            module,
            pass_manager,
            builder,
            fn_stack: Default::default(),
            visited_defers: Default::default(),
        }
    }

    pub fn gen_module(mut self) -> Module<'a> {
        for extern_fn_ast in self.package.extern_fns.iter() {
            self.add_extern_fn(extern_fn_ast);
        }

        for fn_ast in self.package.fns.iter() {
            self.add_fn(fn_ast);
        }

        for fn_ast in self.package.fns.iter() {
            self.gen_fn(fn_ast);
        }

        self.module
    }

    pub fn gen_defers(&mut self, first_frame_id: FrameId, last_frame_id: FrameId) {
        let mut current_frame_id = last_frame_id;

        while current_frame_id != first_frame_id {
            if let Some(stmt) = self.package.defer_stmts.get(current_frame_id) {
                // Generate defers only if they were not generated before
                if self.visited_defers.insert(current_frame_id) {
                    self.gen_stmt(stmt);
                }
            }

            current_frame_id = self.resolve.get_parent_frame(current_frame_id).unwrap();
        }

        self.visited_defers.clear();
    }

    pub fn gen_return_defers(&mut self, return_frame_id: FrameId) {
        let mut current_frame_id = return_frame_id;

        loop {
            if let Some(stmt) = self.package.defer_stmts.get(current_frame_id) {
                // Generate defers only if they were not generated before
                if self.visited_defers.insert(current_frame_id) {
                    self.gen_stmt(stmt);
                }
            }

            current_frame_id = match self.resolve.get_parent_frame(current_frame_id) {
                Some(frame_id) => frame_id,
                None => break,
            }
        }

        self.visited_defers.clear();
    }

    #[inline]
    pub fn append_block_after(&self, block: BasicBlock<'a>) -> BasicBlock<'a> {
        self.context.insert_basic_block_after(block, "")
    }

    #[inline]
    pub fn append_block_after_current_block(&self) -> BasicBlock<'a> {
        let current_block = self.builder.current_block();
        self.context.insert_basic_block_after(current_block, "")
    }

    pub fn get_fn_entry_block(&self) -> BasicBlock<'a> {
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
}
