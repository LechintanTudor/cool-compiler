use cool_parser::FnCallExpr;
use cool_resolve::{ExprId, TyId, FrameId, tys};
use crate::{ExprAst, AstGenerator, AstResult, TyNotFn};

#[derive(Clone, Debug)]
pub struct FnCallExprAst {
    pub expr_id: ExprId,
    pub fn_expr: Box<ExprAst>,
    pub arg_exprs: Vec<ExprAst>,
}

impl AstGenerator<'_> {
    pub fn gen_fn_call_expr(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        fn_call_expr: &FnCallExpr, 
    ) -> AstResult<FnCallExprAst> {
        let fn_expr = self.gen_expr(frame_id, tys::INFERRED, &fn_call_expr.fn_expr)?;
        let fn_expr_ty_id = self.resolve[fn_expr.id()];
        let fn_ty = self
            .resolve[fn_expr_ty_id]
            .as_fn_ty()
            .ok_or(TyNotFn {
                found_ty: fn_expr_ty_id,
            })?;
        
        todo!("check param count and variadicity")
    }
}
