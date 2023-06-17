mod cond_block;
mod defer_code_map;
mod error;
mod expr;
mod expr_or_stmt;
mod function;
mod package;
mod resolve;
mod stmt;

pub use self::cond_block::*;
pub use self::defer_code_map::*;
pub use self::error::*;
pub use self::expr::*;
pub use self::expr_or_stmt::*;
pub use self::function::*;
pub use self::package::*;
pub use self::resolve::*;
pub use self::stmt::*;
use cool_resolve::{ResolveContext, TyConsts, TyId};
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct FnState {
    pub ret: TyId,
}

pub struct AstGenerator<'a> {
    pub resolve: &'a mut ResolveContext,
    pub defer_codes: DeferStmtMap,
    fn_state_stack: Vec<FnState>,
}

impl<'a> AstGenerator<'a> {
    #[inline]
    pub fn new(resolve: &'a mut ResolveContext) -> Self {
        Self {
            resolve,
            defer_codes: Default::default(),
            fn_state_stack: Default::default(),
        }
    }

    #[inline]
    pub fn tys(&self) -> &TyConsts {
        self.resolve.ty_consts()
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

    pub fn resolve_direct_ty_id(
        &self,
        span: Span,
        found_ty_id: TyId,
        expected_ty_id: TyId,
    ) -> AstResult<TyId> {
        self.resolve
            .resolve_direct_ty_id(found_ty_id, expected_ty_id)
            .map_err(|error| {
                AstError::new(
                    span,
                    TyError {
                        ty_id: error.found_ty_id,
                        kind: TyErrorKind::TyMismatch {
                            expected_ty_id: error.expected_ty_id,
                        },
                    },
                )
            })
    }
}
