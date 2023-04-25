use crate::{AstGenerator, AstResult};
use cool_parser::IdentExpr;
use cool_resolve::{BindingId, ExprId, FrameId};

#[derive(Clone, Debug)]
pub struct IdentExprAst {
    pub expr_id: ExprId,
    pub binding_id: BindingId,
}

impl AstGenerator<'_> {
    pub fn gen_ident_expr(
        &mut self,
        frame_id: FrameId,
        ident_expr: &IdentExpr,
    ) -> AstResult<IdentExprAst> {
        let binding_id = self
            .resolve
            .resolve_local(frame_id, ident_expr.ident.symbol)?
            .as_binding_id()
            .unwrap();

        let expr_id = self.resolve.add_expr(self.resolve[binding_id].ty_id);

        Ok(IdentExprAst {
            expr_id,
            binding_id,
        })
    }
}
