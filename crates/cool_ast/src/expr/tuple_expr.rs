use crate::{AstGenerator, AstResult, ExprAst};
use cool_parser::TupleExpr;
use cool_resolve::{ExprId, FrameId, ResolveExpr, TupleTy, TyId};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct TupleExprAst {
    pub span: Span,
    pub expr_id: ExprId,
    pub elems: Vec<ExprAst>,
}

impl Section for TupleExprAst {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl AstGenerator<'_> {
    pub fn gen_tuple_expr(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        expr: &TupleExpr,
    ) -> AstResult<ExprAst> {
        let elems = match expected_ty_id.as_tuple() {
            Some(tuple_ty) => self.gen_tuple_elems_with_type(frame_id, tuple_ty, expr)?,
            None => self.gen_tuple_elems_without_type(frame_id, expr)?,
        };

        let found_ty_id = self
            .resolve
            .mk_tuple(elems.iter().map(|elem| elem.expr_id().ty_id));

        self.resolve_expr(
            expr.span(),
            found_ty_id,
            expected_ty_id,
            |resolve, span, ty_id| {
                TupleExprAst {
                    span,
                    expr_id: resolve.add_expr(ResolveExpr::rvalue(ty_id)),
                    elems,
                }
            },
        )
    }

    fn gen_tuple_elems_without_type(
        &mut self,
        frame_id: FrameId,
        expr: &TupleExpr,
    ) -> AstResult<Vec<ExprAst>> {
        expr.elems
            .iter()
            .map(|elem| self.gen_expr(frame_id, self.tys().infer, elem))
            .collect::<Result<Vec<_>, _>>()
    }

    fn gen_tuple_elems_with_type(
        &mut self,
        frame_id: FrameId,
        tuple_ty: &TupleTy,
        expr: &TupleExpr,
    ) -> AstResult<Vec<ExprAst>> {
        expr.elems
            .iter()
            .zip(tuple_ty.elems().iter().copied())
            .map(|(elem, elem_ty_id)| self.gen_expr(frame_id, elem_ty_id, elem))
            .collect::<Result<Vec<_>, _>>()
    }
}
