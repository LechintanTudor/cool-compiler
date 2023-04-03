use crate::expr::{ExprAst, GenericExprAst};
use crate::{AstGenerator, Unify};
use cool_parser::FnCallExpr;
use cool_resolve::expr_ty::{ExprId, ExprTyUnifier};
use cool_resolve::resolve::ScopeId;
use cool_resolve::ty::TyTable;

#[derive(Clone, Debug)]
pub struct FnCallExprAst {
    pub id: ExprId,
    pub fn_expr: Box<ExprAst>,
    pub arg_exprs: Vec<ExprAst>,
}

impl Unify for FnCallExprAst {
    fn unify(&self, _unififer: &mut ExprTyUnifier, _tys: &mut TyTable) {
        todo!()
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
            id: self.unification.add_expr(),
            fn_expr: Box::new(fn_expr),
            arg_exprs,
        }
    }
}
