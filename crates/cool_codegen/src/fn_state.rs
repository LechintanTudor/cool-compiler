use crate::CodeGenerator;
use cool_resolve::FrameId;
use inkwell::basic_block::BasicBlock;
use inkwell::values::{FunctionValue, InstructionValue};

#[derive(Clone, Copy, Debug)]
pub struct JumpBlock<'a> {
    pub first_frame_id: FrameId,
    pub break_block: BasicBlock<'a>,
    pub continue_block: BasicBlock<'a>,
}

#[derive(Clone, Debug)]
pub struct FnState<'a> {
    pub fn_value: FunctionValue<'a>,
    pub last_alloca: Option<InstructionValue<'a>>,
    pub jump_blocks: Vec<JumpBlock<'a>>,
}

impl<'a> FnState<'a> {
    #[inline]
    pub fn new(fn_value: FunctionValue<'a>) -> Self {
        Self {
            fn_value,
            last_alloca: None,
            jump_blocks: vec![],
        }
    }
}

impl<'a> CodeGenerator<'a> {
    #[inline]
    pub fn fn_state(&self) -> &FnState<'a> {
        self.fn_stack.last().unwrap()
    }

    #[inline]
    pub fn fn_state_mut(&mut self) -> &mut FnState<'a> {
        self.fn_stack.last_mut().unwrap()
    }

    #[inline]
    pub fn push_jump_block(&mut self, jump_block: JumpBlock<'a>) {
        self.fn_state_mut().jump_blocks.push(jump_block);
    }

    #[inline]
    pub fn pop_jump_block(&mut self) {
        self.fn_state_mut().jump_blocks.pop().unwrap();
    }

    #[inline]
    pub fn jump_block(&self) -> &JumpBlock<'a> {
        self.fn_state().jump_blocks.last().unwrap()
    }
}
