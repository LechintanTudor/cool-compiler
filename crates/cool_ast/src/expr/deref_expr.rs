use crate::{AstError, AstGenerator, AstResult, ExprAst};
use cool_parser::DerefExpr;
use cool_resolve::{AnyTy, ExprId, FrameId, ResolveExpr, TyId, ValueTy};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct DerefExprAst {
    pub span: Span,
    pub expr_id: ExprId,
    pub expr: Box<ExprAst>,
}

impl Section for DerefExprAst {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl AstGenerator<'_> {
    pub fn gen_deref_expr(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        deref_expr: &DerefExpr,
    ) -> AstResult<DerefExprAst> {
        let expr = self.gen_expr(frame_id, expected_ty_id, &deref_expr.expr)?;
        let expr_ty_id = self.resolve[expr.expr_id()].ty_id;

        let AnyTy::Value(ValueTy::Ptr(pointer_ty)) = &*expr_ty_id else {
            Err(AstError::TyNotPointer)?
        };

        let ty_id = self
            .resolve
            .resolve_direct_ty_id(pointer_ty.pointee, expected_ty_id)?;

        let expr_id = self
            .resolve
            .add_expr(ResolveExpr::lvalue(ty_id, pointer_ty.is_mutable));

        Ok(DerefExprAst {
            span: deref_expr.span,
            expr_id,
            expr: Box::new(expr),
        })
    }
}
