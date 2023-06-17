use crate::{AstError, AstGenerator, AstResult, ExprAst};
use cool_parser::DeclStmt;
use cool_resolve::{BindingId, FrameId};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct DeclStmtAst {
    pub span: Span,
    pub frame_id: FrameId,
    pub binding_id: BindingId,
    pub expr: Box<ExprAst>,
}

impl Section for DeclStmtAst {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl AstGenerator<'_> {
    pub fn gen_decl_stmt(
        &mut self,
        frame_id: FrameId,
        decl_stmt: &DeclStmt,
    ) -> AstResult<DeclStmtAst> {
        let expected_ty_id = decl_stmt
            .ty
            .as_ref()
            .map(|ty| self.resolve_ty(frame_id.into(), ty))
            .transpose()?
            .unwrap_or(self.tys().infer);

        let expr = self.gen_expr(frame_id, expected_ty_id, &decl_stmt.expr)?;

        let frame_id = self.resolve.add_frame(frame_id.into());

        let binding_id = self
            .resolve
            .insert_local_binding(
                frame_id,
                decl_stmt.pattern.is_mutable,
                decl_stmt.pattern.ident.symbol,
                Some(self.resolve[expr.expr_id()].ty_id),
            )
            .map_err(|error| AstError::new(decl_stmt.span(), error))?;

        Ok(DeclStmtAst {
            span: decl_stmt.span(),
            frame_id,
            binding_id,
            expr: Box::new(expr),
        })
    }
}
