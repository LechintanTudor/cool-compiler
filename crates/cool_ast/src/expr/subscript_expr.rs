use crate::{AstGenerator, AstResult, ExprAst};
use cool_parser::SubscriptExpr;
use cool_resolve::{tys, ExprId, FrameId, ResolveExpr, TyId, ValueTy};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct SubscriptExprAst {
    pub span: Span,
    pub expr_id: ExprId,
    pub base: Box<ExprAst>,
    pub index: Box<ExprAst>,
}

impl Section for SubscriptExprAst {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl AstGenerator<'_> {
    pub fn gen_subscript_expr(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        expr: &SubscriptExpr,
    ) -> AstResult<SubscriptExprAst> {
        let base = self.gen_expr(frame_id, tys::INFER, &expr.base)?;
        let index = self.gen_expr(frame_id, tys::USIZE, &expr.subscript)?;

        let base_resolve_expr = self.resolve[base.expr_id()];

        let ValueTy::Array(array_ty) = self.resolve[base_resolve_expr.ty_id].ty else {
            panic!("ty not array");
        };

        let ty_id = self
            .resolve
            .resolve_direct_ty_id(array_ty.elem, expected_ty_id)?;

        let expr_id = self.resolve.add_expr(ResolveExpr {
            ty_id,
            kind: base_resolve_expr.kind,
        });

        Ok(SubscriptExprAst {
            span: expr.span,
            expr_id,
            base: Box::new(base),
            index: Box::new(index),
        })
    }
}
