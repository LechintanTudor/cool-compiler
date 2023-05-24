mod cond_block;
mod error;
mod expr;
mod function;
mod package;
mod resolve;
mod stmt;

pub use self::cond_block::*;
pub use self::error::*;
pub use self::expr::*;
pub use self::function::*;
pub use self::package::*;
pub use self::resolve::*;
pub use self::stmt::*;
use cool_resolve::{ResolveContext, TyId};

#[derive(Clone, Debug)]
pub struct FnState {
    pub ret: TyId,
}

pub struct AstGenerator<'a> {
    pub resolve: &'a mut ResolveContext,
    fn_state_stack: Vec<FnState>,
}

impl<'a> AstGenerator<'a> {
    #[inline]
    pub fn new(resolve: &'a mut ResolveContext) -> Self {
        Self {
            resolve,
            fn_state_stack: Default::default(),
        }
    }

    #[inline]
    pub fn fn_state(&self) -> &FnState {
        self.fn_state_stack.last().unwrap()
    }

    #[inline]
    pub fn push_fn_state(&mut self, fn_state: FnState) {
        self.fn_state_stack.push(fn_state);
    }

    #[inline]
    pub fn pop_fn_state(&mut self) {
        self.fn_state_stack.pop();
    }
}
