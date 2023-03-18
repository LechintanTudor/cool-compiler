use crate::expr::ExprAst;
use crate::AstGenerator;
use cool_parser::FnCallExpr;
use cool_resolve::binding::FrameId;
use cool_resolve::item::ItemId;

#[derive(Clone, Debug)]
pub struct FnCallExprAst {
    pub fn_expr: Box<ExprAst>,
    pub arg_exprs: Vec<ExprAst>,
}

impl AstGenerator<'_> {
    pub fn generate_fn_call_expr(
        &mut self,
        module_id: ItemId,
        parent_id: Option<FrameId>,
        fn_call: &FnCallExpr,
    ) -> FnCallExprAst {
        let fn_expr = self.generate_expr(module_id, parent_id, &fn_call.fn_expr);
        let arg_exprs = fn_call
            .arg_exprs
            .iter()
            .map(|arg| self.generate_expr(module_id, parent_id, arg))
            .collect::<Vec<_>>();

        FnCallExprAst {
            fn_expr: Box::new(fn_expr),
            arg_exprs,
        }
    }
}
