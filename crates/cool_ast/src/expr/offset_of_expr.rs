use crate::{AstError, AstGenerator, AstResult};
use cool_parser::OffsetOfExpr;
use cool_resolve::{ExprId, FrameId, ResolveExpr, TyId};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct OffsetOfExprAst {
    pub span: Span,
    pub expr_id: ExprId,
    pub value: u64,
}

impl Section for OffsetOfExprAst {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl AstGenerator<'_> {
    pub fn gen_offset_of_expr(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        expr: &OffsetOfExpr,
    ) -> AstResult<OffsetOfExprAst> {
        let mut arg_ty_id = self.resolve_ty(frame_id, &expr.ty)?;
        let mut offset = 0;

        for symbol in expr.path.idents.iter().map(|ident| ident.symbol) {
            let field = self
                .resolve
                .get_ty_def(arg_ty_id)
                .and_then(|def| def.get_aggregate_field(symbol))
                .ok_or(AstError::field_not_found(expr.span, arg_ty_id, symbol))?;

            arg_ty_id = field.ty_id;
            offset += field.offset;
        }

        let ty_id = self.resolve_direct_ty_id(expr.span, self.tys().i64, expected_ty_id)?;

        Ok(OffsetOfExprAst {
            span: expr.span,
            expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
            value: offset,
        })
    }
}
