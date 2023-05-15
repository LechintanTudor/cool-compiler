use crate::{
    AstGenerator, AstResult, ExprAst, ExprNotAddressable, ExprNotMutablyAddressable, TyMismatch,
};
use cool_parser::{UnaryExpr, UnaryOpKind};
use cool_resolve::{tys, ExprId, FrameId, ResolveExpr, TyId};

#[derive(Clone, Debug)]
pub struct UnaryExprAst {
    pub expr_id: ExprId,
    pub op: UnaryOpKind,
    pub expr: Box<ExprAst>,
}

impl AstGenerator<'_> {
    pub fn gen_unary_expr(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        unary_expr: &UnaryExpr,
    ) -> AstResult<UnaryExprAst> {
        let expr = match unary_expr.op.kind {
            UnaryOpKind::Minus => {
                let expr = self.gen_expr(frame_id, expected_ty_id, &unary_expr.expr)?;
                let ty_id = self.resolve[expr.id()].ty_id;

                if !ty_id.is_number() {
                    Err(TyMismatch {
                        found: ty_id,
                        expected: tys::INFERRED_NUMBER,
                    })?
                }

                UnaryExprAst {
                    expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
                    op: UnaryOpKind::Minus,
                    expr: Box::new(expr),
                }
            }
            UnaryOpKind::Not => {
                let expr = self.gen_expr(frame_id, expected_ty_id, &unary_expr.expr)?;
                let ty_id = self.resolve[expr.id()].ty_id;

                if !ty_id.is_number() && ty_id != tys::BOOL {
                    Err(TyMismatch {
                        found: ty_id,
                        expected: tys::INFERRED_NUMBER,
                    })?
                }

                UnaryExprAst {
                    expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
                    op: UnaryOpKind::Not,
                    expr: Box::new(expr),
                }
            }
            UnaryOpKind::Addr { is_mutable } => {
                let inner_expr = self.gen_expr(frame_id, tys::INFERRED, &unary_expr.expr)?;
                let inner_resolve_expr = self.resolve[inner_expr.id()];

                let ty_id = self
                    .resolve
                    .mk_pointer(is_mutable, inner_resolve_expr.ty_id);

                let ty_id = ty_id
                    .resolve_non_inferred(expected_ty_id)
                    .ok_or(TyMismatch {
                        found: ty_id,
                        expected: expected_ty_id,
                    })?;

                if is_mutable {
                    if !inner_resolve_expr.is_mutably_addressable() {
                        Err(ExprNotMutablyAddressable)?
                    }
                } else {
                    if !inner_resolve_expr.is_addressable() {
                        Err(ExprNotAddressable)?
                    }
                }

                UnaryExprAst {
                    expr_id: self
                        .resolve
                        .add_expr(ResolveExpr::lvalue(ty_id, is_mutable)),
                    op: UnaryOpKind::Addr { is_mutable },
                    expr: Box::new(inner_expr),
                }
            }
        };

        Ok(expr)
    }
}
