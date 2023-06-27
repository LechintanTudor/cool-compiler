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
    ) -> AstResult<UnaryExprAst> {
        let expr = match unary_expr.op.kind {
            UnaryOpKind::Minus => {
                let expr = self.gen_expr(frame_id, expected_ty_id, &unary_expr.expr)?;
                let ty_id = self.resolve_direct_ty_id(
                    unary_expr.span(),
                    self.resolve[expr.expr_id()].ty_id,
                    self.tys().infer_number,
                )?;

                UnaryExprAst {
                    expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
                    op: unary_expr.op,
                    expr: Box::new(expr),
                }
            }
            UnaryOpKind::Not => {
                let expr = self.gen_expr(frame_id, expected_ty_id, &unary_expr.expr)?;
                let ty_id = self.resolve[expr.expr_id()].ty_id;

                if !ty_id.is_number() && ty_id != self.tys().bool {
                    return AstResult::ty_mismatch(
                        unary_expr.span(),
                        ty_id,
                        self.tys().infer_number,
                    );
                }

                UnaryExprAst {
                    expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
                    op: unary_expr.op,
                    expr: Box::new(expr),
                }
            }
            UnaryOpKind::Addr { is_mutable } => {
                let inner_expr = self.gen_expr(frame_id, self.tys().infer, &unary_expr.expr)?;
                let inner_resolve_expr = self.resolve[inner_expr.expr_id()];

                let ty_id = self.resolve.mk_ptr(inner_resolve_expr.ty_id, is_mutable);
                let ty_id = self.resolve_direct_ty_id(unary_expr.span(), ty_id, expected_ty_id)?;

                if is_mutable {
                    if !inner_resolve_expr.is_mutably_addressable() {
                        return AstResult::error(
                            unary_expr.span(),
                            ExprError::NotAddressableMutably,
                        );
                    }
                } else {
                    if !inner_resolve_expr.is_addressable() {
                        return AstResult::error(unary_expr.span(), ExprError::NotAddressable);
                    }
                }

                let expr_id = self
                    .resolve
                    .add_expr(ResolveExpr::lvalue(ty_id, is_mutable));

                UnaryExprAst {
                    expr_id,
                    op: unary_expr.op,
                    expr: Box::new(inner_expr),
                }
            }
        };

        Ok(expr)
    }
}
