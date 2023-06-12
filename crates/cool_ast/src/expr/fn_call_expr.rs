use crate::{AstGenerator, AstResult, ExprAst, FnParamCountMismatch, TyNotFn};
use cool_parser::FnCallExpr;
use cool_resolve::{ExprId, FrameId, ResolveExpr, TyId};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct FnCallExprAst {
    pub span: Span,
    pub expr_id: ExprId,
    pub fn_expr: Box<ExprAst>,
    pub arg_exprs: Vec<ExprAst>,
}

impl Section for FnCallExprAst {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl AstGenerator<'_> {
    pub fn gen_fn_call_expr(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        fn_call_expr: &FnCallExpr,
    ) -> AstResult<FnCallExprAst> {
        let fn_expr = self.gen_expr(frame_id, self.tys().infer, &fn_call_expr.base)?;
        let fn_expr_ty_id = self.resolve[fn_expr.expr_id()].ty_id;
        let fn_ty = fn_expr_ty_id
            .as_fn()
            .ok_or(TyNotFn {
                found: fn_expr_ty_id,
            })?
            .clone();

        if fn_ty.is_variadic {
            if fn_call_expr.args.len() < fn_ty.params.len() {
                Err(FnParamCountMismatch {
                    found: fn_call_expr.args.len() as _,
                    expected: fn_ty.params.len() as _,
                })?;
            }
        } else {
            if fn_call_expr.args.len() != fn_ty.params.len() {
                Err(FnParamCountMismatch {
                    found: fn_call_expr.args.len() as _,
                    expected: fn_ty.params.len() as _,
                })?;
            }
        }

        let mut arg_exprs = Vec::<ExprAst>::new();

        for (i, arg_expr) in fn_call_expr.args.iter().enumerate() {
            let param_ty_id = fn_ty.params.get(i).copied().unwrap_or(self.tys().infer);
            arg_exprs.push(self.gen_expr(frame_id, param_ty_id, arg_expr)?);
        }

        let ret_ty_id = self
            .resolve
            .resolve_direct_ty_id(fn_ty.ret, expected_ty_id)?;

        let expr_id = self.resolve.add_expr(ResolveExpr::rvalue(ret_ty_id));

        Ok(FnCallExprAst {
            span: fn_call_expr.span,
            expr_id,
            fn_expr: Box::new(fn_expr),
            arg_exprs,
        })
    }
}
