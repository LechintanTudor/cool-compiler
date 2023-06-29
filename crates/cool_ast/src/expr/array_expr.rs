use crate::{AstGenerator, AstResult, ExprAst};
use cool_parser::{ArrayExpr, ArrayRepeatExpr};
use cool_resolve::{ExprId, FrameId, ResolveExpr, TyId};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct ArrayExprAst {
    pub span: Span,
    pub expr_id: ExprId,
    pub elems: Vec<ExprAst>,
}

impl Section for ArrayExprAst {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone, Debug)]
pub struct ArrayRepeatExprAst {
    pub span: Span,
    pub expr_id: ExprId,
    pub len: u64,
    pub elem: Box<ExprAst>,
}

impl Section for ArrayRepeatExprAst {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl AstGenerator<'_> {
    pub fn gen_array_expr(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        expr: &ArrayExpr,
    ) -> AstResult<ArrayExprAst> {
        let (ty_id, elems) = match expr.elems.split_first() {
            Some((first_elem, other_elems)) => {
                let expected_elem_ty_id = self.get_expected_array_elem_ty_id(expected_ty_id);
                let first_elem = self.gen_expr(frame_id, expected_elem_ty_id, first_elem)?;
                let elem_ty_id = first_elem.expr_id().ty_id;

                let mut elems = vec![first_elem];
                for elem in other_elems {
                    elems.push(self.gen_expr(frame_id, elem_ty_id, elem)?);
                }

                let ty_id = self.resolve.mk_array(elems.len() as u64, elem_ty_id);
                (ty_id, elems)
            }
            None => (self.tys().infer_empty_array, vec![]),
        };

        let ty_id = self.resolve_direct_ty_id(expr.span(), ty_id, expected_ty_id)?;

        Ok(ArrayExprAst {
            span: expr.span,
            expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
            elems,
        })
    }

    pub fn gen_array_repeat_expr(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        expr: &ArrayRepeatExpr,
    ) -> AstResult<ArrayRepeatExprAst> {
        let len_expr = self.gen_literal_expr(frame_id, self.tys().usize, &expr.len)?;
        let len = len_expr.as_int_value().unwrap() as u64;

        let expected_elem_ty_id = self.get_expected_array_elem_ty_id(expected_ty_id);
        let elem = self.gen_expr(frame_id, expected_elem_ty_id, &expr.elem)?;
        let elem_ty_id = elem.expr_id().ty_id;

        let ty_id = self.resolve.mk_array(len, elem_ty_id);
        let ty_id = self.resolve_direct_ty_id(expr.span(), ty_id, expected_ty_id)?;

        Ok(ArrayRepeatExprAst {
            span: expr.span,
            expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
            len,
            elem: Box::new(elem),
        })
    }

    fn get_expected_array_elem_ty_id(&self, expected_array_ty_id: TyId) -> TyId {
        if expected_array_ty_id.is_infer() {
            return self.tys().infer;
        }

        expected_array_ty_id
            .as_array()
            .map(|array_ty| array_ty.elem)
            .unwrap_or(self.tys().infer)
    }
}
