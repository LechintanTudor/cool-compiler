use crate::expr::{ExprAst, GenericExprAst};
use crate::{AstGenerator, InvalidArgCount, ResolveAst, SemanticResult, TyMismatch, TyNotFn};
use cool_parser::FnCallExpr;
use cool_resolve::{tys, ExprId, ScopeId, TyId};

#[derive(Clone, Debug)]
pub struct FnCallExprAst {
    pub id: ExprId,
    pub fn_expr: Box<ExprAst>,
    pub arg_exprs: Vec<ExprAst>,
}

impl ResolveAst for FnCallExprAst {
    fn resolve(&self, ast: &mut AstGenerator, expected_ty: TyId) -> SemanticResult<TyId> {
        let fn_expr_ty = self.fn_expr.resolve(ast, tys::INFERRED)?;
        let fn_expr_ty_kind = ast.resolve[fn_expr_ty]
            .as_fn_ty()
            .ok_or(TyNotFn {
                found_ty: fn_expr_ty,
            })?
            .clone();

        if self.arg_exprs.len() != fn_expr_ty_kind.params.len() {
            Err(InvalidArgCount {
                found: self.arg_exprs.len() as _,
                expected: fn_expr_ty_kind.params.len() as _,
            })?
        }

        for (arg_expr, &param_ty) in self.arg_exprs.iter().zip(fn_expr_ty_kind.params.iter()) {
            arg_expr.resolve(ast, param_ty)?;
        }

        let expr_ty = fn_expr_ty_kind
            .ret
            .resolve_non_inferred(expected_ty)
            .ok_or(TyMismatch {
                found_ty: fn_expr_ty_kind.ret,
                expected_ty,
            })?;

        ast.resolve.set_expr_ty(self.id, expr_ty);
        Ok(expr_ty)
    }
}

impl GenericExprAst for FnCallExprAst {
    #[inline]
    fn id(&self) -> ExprId {
        self.id
    }
}

impl AstGenerator<'_> {
    pub fn gen_fn_call_expr(&mut self, scope_id: ScopeId, fn_call: &FnCallExpr) -> FnCallExprAst {
        let fn_expr = self.gen_expr(scope_id, &fn_call.fn_expr);
        let arg_exprs = fn_call
            .arg_exprs
            .iter()
            .map(|arg| self.gen_expr(scope_id, arg))
            .collect::<Vec<_>>();

        FnCallExprAst {
            id: self.resolve.add_expr(),
            fn_expr: Box::new(fn_expr),
            arg_exprs,
        }
    }
}
