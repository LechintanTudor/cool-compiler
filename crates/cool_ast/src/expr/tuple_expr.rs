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
    ) -> AstResult<TupleExprAst> {
        let elems = match expected_ty_id.as_tuple() {
            Some(tuple_ty) => self.gen_tuple_elems_with_type(frame_id, tuple_ty, expr)?,
            None => self.gen_tuple_elems_without_type(frame_id, expr)?,
        };

        let ty_id = {
            let elem_ty_ids = elems
                .iter()
                .map(|elem| self.resolve.get_expr_ty_id(elem.expr_id()))
                .collect::<Vec<_>>();

            let ty_id = self.resolve.mk_tuple(elem_ty_ids);
            self.resolve_direct_ty_id(expr.span(), ty_id, expected_ty_id)?
        };

        Ok(TupleExprAst {
            span: expr.span,
            expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
            elems,
        })
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
            .zip(tuple_ty.elems.iter().copied())
            .map(|(elem, elem_ty_id)| self.gen_expr(frame_id, elem_ty_id, elem))
            .collect::<Result<Vec<_>, _>>()
    }
}
