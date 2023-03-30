use crate::expr::ExprAst;
use crate::AstGenerator;
use cool_parser::FnCallExpr;
use cool_resolve::resolve::ScopeId;

#[derive(Clone, Debug)]
pub struct FnCallExprAst {
    pub fn_expr: Box<ExprAst>,
    pub arg_exprs: Vec<ExprAst>,
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
            fn_expr: Box::new(fn_expr),
            arg_exprs,
        }
    }
}
