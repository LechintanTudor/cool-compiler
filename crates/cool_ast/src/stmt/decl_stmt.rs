use crate::{resolve_ty, AstGenerator, ExprAst, SpannedAstResult};
use cool_derive::Section;
use cool_parser::DeclStmt;
use cool_resolve::{tys, Binding, FrameId};
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct DeclStmtAst {
    pub span: Span,
    pub frame_id: FrameId,
    pub expr: Box<ExprAst>,
}

impl AstGenerator<'_> {
    pub fn gen_decl_stmt(
        &mut self,
        stmt: &DeclStmt,
        frame_id: FrameId,
    ) -> SpannedAstResult<DeclStmtAst> {
        let expected_ty_id = stmt
            .ty
            .as_ref()
            .map(|ty| resolve_ty(self.context, frame_id, ty))
            .transpose()?
            .unwrap_or(tys::infer);

        let expr = self.gen_expr(&stmt.expr, frame_id, expected_ty_id)?;

        let frame_id = self.context.add_frame(frame_id);

        self.context
            .add_binding(
                frame_id,
                Binding {
                    is_mutable: stmt.pattern.is_mutable,
                    symbol: stmt.pattern.ident.symbol,
                    ty_id: self.context[expr.expr_id()].ty_id,
                },
            )
            .unwrap();

        Ok(DeclStmtAst {
            span: stmt.span(),
            frame_id,
            expr: Box::new(expr),
        })
    }
}
