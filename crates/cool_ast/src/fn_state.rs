use crate::{AstError, AstGenerator, AstResult, LogicError};
use cool_resolve::TyId;
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct FnState {
    pub ret_ty_id: TyId,
    pub main_block_ty_ids: Vec<TyId>,
}

impl AstGenerator<'_> {
    #[inline]
    pub fn fn_state(&self) -> &FnState {
        self.fn_states.last().unwrap()
    }

    #[inline]
    pub fn fn_state_mut(&mut self) -> &mut FnState {
        self.fn_states.last_mut().unwrap()
    }

    #[inline]
    pub fn fn_ret_ty_id(&self) -> TyId {
        self.fn_state().ret_ty_id
    }

    #[inline]
    pub fn push_fn_state(&mut self, ret_ty_id: TyId) {
        self.fn_states.push(FnState {
            ret_ty_id,
            main_block_ty_ids: vec![],
        });
    }

    #[inline]
    pub fn pop_fn_state(&mut self) {
        self.fn_states.pop();
    }

    #[inline]
    pub fn block_ty_id(&self, span: Span) -> AstResult<TyId> {
        self.fn_state()
            .main_block_ty_ids
            .last()
            .copied()
            .ok_or_else(|| AstError::new(span, LogicError::InvalidJump))
    }

    #[inline]
    pub fn push_block_ty_id(&mut self, block_ty_id: TyId) {
        self.fn_state_mut().main_block_ty_ids.push(block_ty_id)
    }

    #[inline]
    pub fn pop_block_ty_id(&mut self) {
        self.fn_state_mut().main_block_ty_ids.pop().unwrap();
    }
}
