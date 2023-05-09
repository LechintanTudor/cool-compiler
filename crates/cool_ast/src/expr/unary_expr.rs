use crate::{AstGenerator, AstResult, ExprAst, TyMismatch};
use cool_parser::{UnaryExpr, UnaryOpKind};
use cool_resolve::{tys, ExprId, FrameId, TyId};

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
        let (ty_id, expr) = match unary_expr.op.kind {
            UnaryOpKind::Minus => {
                let expr = self.gen_expr(frame_id, expected_ty_id, &unary_expr.expr)?;
                let ty_id = self.resolve[expr.id()].ty_id;

                if !ty_id.is_number() {
                    Err(TyMismatch {
                        found: ty_id,
                        expected: tys::INFERRED_NUMBER,
                    })?
                }

                (ty_id, expr)
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

                (ty_id, expr)
            }
            UnaryOpKind::Addr { is_mutable } => {
                let expr = self.gen_expr(frame_id, tys::INFERRED, &unary_expr.expr)?;
                let expr_ty_id = self.resolve[expr.id()].ty_id;
                let ty_id = self.resolve.mk_pointer(is_mutable, expr_ty_id);

                let ty_id = ty_id
                    .resolve_non_inferred(expected_ty_id)
                    .ok_or(TyMismatch {
                        found: ty_id,
                        expected: expected_ty_id,
                    })?;

                (ty_id, expr)
            }
        };

        Ok(UnaryExprAst {
            expr_id: self.resolve.add_expr(ty_id, false),
            op: unary_expr.op.kind,
            expr: Box::new(expr),
        })
    }
}
