mod cond_block;
mod defer_code_map;
mod error;
mod expr;
mod expr_or_stmt;
mod fn_state;
mod function;
mod package;
mod resolve;
mod stmt;

pub use self::cond_block::*;
pub use self::defer_code_map::*;
pub use self::error::*;
pub use self::expr::*;
pub use self::expr_or_stmt::*;
pub use self::fn_state::*;
pub use self::function::*;
pub use self::package::*;
pub use self::resolve::*;
pub use self::stmt::*;
use cool_resolve::{ResolveContext, TyConsts, TyId};
use cool_span::Span;

pub struct AstGenerator<'a> {
    pub resolve: &'a mut ResolveContext,
    pub defer_codes: DeferStmtMap,
    pub fn_states: Vec<FnState>,
}

impl<'a> AstGenerator<'a> {
    #[inline]
    pub fn new(resolve: &'a mut ResolveContext) -> Self {
        Self {
            resolve,
            defer_codes: Default::default(),
            fn_states: Default::default(),
        }
    }

    #[inline]
    pub fn tys(&self) -> &TyConsts {
        self.resolve.ty_consts()
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
