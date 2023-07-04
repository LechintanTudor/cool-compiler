mod cond_block;
mod defer_code_map;
mod error;
mod expr;
mod expr_or_stmt;
mod fn_item;
mod fn_state;
mod package;
mod resolve;
mod stmt;

pub use self::cond_block::*;
pub use self::defer_code_map::*;
pub use self::error::*;
pub use self::expr::*;
pub use self::expr_or_stmt::*;
pub use self::fn_item::*;
pub use self::fn_state::*;
pub use self::package::*;
pub use self::resolve::*;
pub use self::stmt::*;
use cool_resolve::{ExprId, ResolveContext, ResolveExpr, TyConsts, TyId, TyResolutionMethod};
use cool_span::Span;

pub struct AstGenerator<'a> {
    pub resolve: &'a mut ResolveContext,
    pub defer_stmts: DeferStmtMap,
    pub fn_states: Vec<FnState>,
    implicit_unit_expr_id: ExprId,
}

impl<'a> AstGenerator<'a> {
    #[inline]
    pub fn new(resolve: &'a mut ResolveContext) -> Self {
        let unit_ty_id = resolve.ty_consts().unit;
        let implicit_unit_expr_id = resolve.add_expr(ResolveExpr::rvalue(unit_ty_id));

        Self {
            resolve,
            defer_stmts: Default::default(),
            fn_states: Default::default(),
            implicit_unit_expr_id,
        }
    }

    #[inline]
    pub fn tys(&self) -> &TyConsts {
        self.resolve.ty_consts()
    }

    #[inline]
    pub fn implicit_unit_expr(
        &mut self,
        span_start: u32,
        expected_ty_id: TyId,
    ) -> AstResult<ExprAst> {
        let implicit_unit_expr_id = self.implicit_unit_expr_id;

        self.resolve_expr(
            Span::new(span_start, 0),
            self.tys().unit,
            expected_ty_id,
            |_, span, _| {
                UnitExprAst {
                    span,
                    expr_id: implicit_unit_expr_id,
                }
            },
        )
    }

    pub fn resolve_expr<E, F>(
        &mut self,
        span: Span,
        found_ty_id: TyId,
        expected_ty_id: TyId,
        expr_builder: F,
    ) -> AstResult<ExprAst>
    where
        E: Into<ExprAst>,
        F: FnOnce(&mut ResolveContext, Span, TyId) -> E,
    {
        let (ty_id, method) = self
            .resolve
            .resolve_ty_id(found_ty_id, expected_ty_id)
            .ok_or(AstError::new(
                span,
                TyError {
                    ty_id: found_ty_id,
                    kind: TyErrorKind::TyMismatch { expected_ty_id },
                },
            ))?;

        let expr = match method {
            TyResolutionMethod::WrapInVariant { wrapped_ty_id } => {
                let inner = expr_builder(self.resolve, span, wrapped_ty_id).into();
                self.continue_gen_variant_wrap_expr(Box::new(inner), ty_id)?
                    .into()
            }
            _ => expr_builder(self.resolve, span, ty_id).into(),
        };

        Ok(expr)
    }
}
