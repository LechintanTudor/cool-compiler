use crate::{AstGenerator, AstResult, AstResultExt, ExprAst, ExprError};
use cool_parser::{UnaryExpr, UnaryOp, UnaryOpKind};
use cool_resolve::{ExprId, FrameId, ResolveExpr, TyId};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct UnaryExprAst {
    pub expr_id: ExprId,
    pub op: UnaryOp,
    pub expr: Box<ExprAst>,
}

impl Section for UnaryExprAst {
    #[inline]
    fn span(&self) -> Span {
        self.op.span.to(self.expr.span())
    }
}

impl AstGenerator<'_> {
    pub fn gen_unary_expr(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        unary_expr: &UnaryExpr,
    ) -> AstResult<ExprAst> {
        match unary_expr.op.kind {
            UnaryOpKind::Minus => {
                let expr = {
                    let expected_ty_id = expected_ty_id
                        .is_number()
                        .then_some(expected_ty_id)
                        .unwrap_or(self.tys().infer_number);

                    self.gen_expr(frame_id, expected_ty_id, &unary_expr.expr)?
                };

                self.resolve_expr(
                    unary_expr.span(),
                    expr.expr_id().ty_id,
                    expected_ty_id,
                    |resolve, _, ty_id| {
                        UnaryExprAst {
                            expr_id: resolve.add_expr(ResolveExpr::rvalue(ty_id)),
                            op: unary_expr.op,
                            expr: Box::new(expr),
                        }
                    },
                )
            }
            UnaryOpKind::Not => {
                let expr = {
                    let expected_ty_id = (expected_ty_id.is_number() || expected_ty_id.is_bool())
                        .then_some(expected_ty_id)
                        .unwrap_or(self.tys().infer);

                    self.gen_expr(frame_id, expected_ty_id, &unary_expr.expr)?
                };

                self.resolve_expr(
                    unary_expr.span(),
                    expr.expr_id().ty_id,
                    expected_ty_id,
                    |resolve, _, ty_id| {
                        UnaryExprAst {
                            expr_id: resolve.add_expr(ResolveExpr::rvalue(ty_id)),
                            op: unary_expr.op,
                            expr: Box::new(expr),
                        }
                    },
                )
            }
            UnaryOpKind::Addr { is_mutable } => {
                let inner_expr = self.gen_expr(frame_id, self.tys().infer, &unary_expr.expr)?;
                let inner_expr_id = inner_expr.expr_id();

                if is_mutable {
                    if !inner_expr_id.is_mutably_addressable() {
                        return AstResult::error(
                            unary_expr.span(),
                            ExprError::NotAddressableMutably,
                        );
                    }
                } else {
                    if !inner_expr_id.is_addressable() {
                        return AstResult::error(unary_expr.span(), ExprError::NotAddressable);
                    }
                }

                let found_ty_id = self.resolve.mk_ptr(inner_expr_id.ty_id, is_mutable);

                self.resolve_expr(
                    unary_expr.span(),
                    found_ty_id,
                    expected_ty_id,
                    |resolve, _, ty_id| {
                        UnaryExprAst {
                            expr_id: resolve.add_expr(ResolveExpr::rvalue(ty_id)),
                            op: unary_expr.op,
                            expr: Box::new(inner_expr),
                        }
                    },
                )
            }
        }
    }
}
