use crate::{AstGenerator, AstResult, ExprAst};
use cool_parser::DeclStmt;
use cool_resolve::{tys, BindingId, FrameId};

#[derive(Clone, Debug)]
pub struct DeclStmtAst {
    pub frame_id: FrameId,
    pub binding_id: BindingId,
    pub expr: Box<ExprAst>,
}

impl AstGenerator<'_> {
    pub fn gen_decl_stmt(
        &mut self,
        frame_id: FrameId,
        decl_stmt: &DeclStmt,
    ) -> AstResult<DeclStmtAst> {
        let expected_ty_id = match decl_stmt.ty.as_ref() {
            Some(ty) => self.resolve_ty(frame_id.into(), ty)?,
            None => tys::INFER,
        };

        let expr = self
            .gen_expr(frame_id, expected_ty_id, &decl_stmt.expr)?
            .ensure_not_module()?;

        let frame_id = self.resolve.add_frame(frame_id.into());

        let binding_id = self.resolve.insert_local_binding(
            frame_id,
            decl_stmt.pattern.is_mutable,
            decl_stmt.pattern.ident.symbol,
            Some(self.resolve[expr.expr_id()].ty_id),
        )?;

        Ok(DeclStmtAst {
            frame_id,
            binding_id,
            expr: Box::new(expr),
        })
    }
}
