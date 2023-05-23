use crate::{AstGenerator, AstResult, ExprAst};
use cool_parser::ReturnExpr;
use cool_resolve::{tys, ExprId, FrameId, ResolveExpr, TyId};

#[derive(Clone, Debug)]
pub struct ReturnExprAst {
    pub expr_id: ExprId,
    pub expr: Option<Box<ExprAst>>,
}

impl AstGenerator<'_> {
    pub fn gen_return_expr(
        &mut self,
        frame_id: FrameId,
        _expected_ty_id: TyId,
        expr: &ReturnExpr,
    ) -> AstResult<ReturnExprAst> {
        let expr = expr
            .expr
            .as_ref()
            .map(|expr| self.gen_expr(frame_id, self.fn_state().ret, expr))
            .transpose()?;

        Ok(ReturnExprAst {
            expr_id: self.resolve.add_expr(ResolveExpr::rvalue(tys::DIVERGE)),
            expr: expr.map(Box::new),
        })
    }
}
