use crate::{AstGenerator, AstResult, AstResultExt, ExprAst, TyError, TyErrorKind};
use cool_parser::{BinOp, BinaryExpr, BitwiseOp};
use cool_resolve::{ExprId, FrameId, ResolveExpr, TyId};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct BinaryExprAst {
    pub expr_id: ExprId,
    pub bin_op: BinOp,
    pub lhs: Box<ExprAst>,
    pub rhs: Box<ExprAst>,
}

impl Section for BinaryExprAst {
    #[inline]
    fn span(&self) -> Span {
        self.lhs.span().to(self.rhs.span())
    }
}

impl AstGenerator<'_> {
    pub fn gen_binary_expr(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        binary_expr: &BinaryExpr,
    ) -> AstResult<BinaryExprAst> {
        let bin_op = binary_expr.bin_op;

        let (ty_id, lhs, rhs) = match binary_expr.bin_op {
            BinOp::Arithmetic(_) => {
                let lhs = self.gen_expr(frame_id, expected_ty_id, &binary_expr.lhs)?;
                let lhs_ty_id = lhs.expr_id().ty_id;
                let rhs = self.gen_expr(frame_id, lhs_ty_id, &binary_expr.rhs)?;

                let ty_id = self.resolve_ty_id(lhs.span(), lhs_ty_id, self.tys().infer_number)?;

                (ty_id, lhs, rhs)
            }
            BinOp::Comparison(_) => {
                let lhs = self.gen_expr(frame_id, self.tys().infer, &binary_expr.lhs)?;
                let lhs_ty_id = lhs.expr_id().ty_id;
                let rhs = self.gen_expr(frame_id, lhs_ty_id, &binary_expr.rhs)?;

                if !lhs_ty_id.is_comparable() {
                    return AstResult::error(
                        binary_expr.span(),
                        TyError {
                            ty_id: lhs_ty_id,
                            kind: TyErrorKind::TyNotComparable,
                        },
                    );
                }

                let ty_id =
                    self.resolve_ty_id(binary_expr.span(), self.tys().bool, expected_ty_id)?;

                (ty_id, lhs, rhs)
            }
            BinOp::Bitwise(bitwise_op) => {
                let lhs = self.gen_expr(frame_id, expected_ty_id, &binary_expr.lhs)?;
                let lhs_ty_id = lhs.expr_id().ty_id;

                let rhs_expected_ty_id = if lhs_ty_id == self.tys().bool {
                    if matches!(bitwise_op, BitwiseOp::Shl | BitwiseOp::Shr) {
                        return AstResult::ty_mismatch(
                            binary_expr.span(),
                            self.tys().bool,
                            self.tys().infer_int,
                        );
                    }

                    self.tys().bool
                } else if lhs_ty_id.is_int() {
                    match bitwise_op {
                        BitwiseOp::Shl | BitwiseOp::Shr => self.tys().infer_int,
                        _ => lhs_ty_id,
                    }
                } else {
                    return AstResult::ty_mismatch(
                        binary_expr.span(),
                        lhs_ty_id,
                        self.tys().infer_int,
                    );
                };

                let rhs = self.gen_expr(frame_id, rhs_expected_ty_id, &binary_expr.rhs)?;

                (lhs_ty_id, lhs, rhs)
            }
            BinOp::Logical(_) => {
                let lhs = self.gen_expr(frame_id, self.tys().bool, &binary_expr.lhs)?;
                let rhs = self.gen_expr(frame_id, self.tys().bool, &binary_expr.rhs)?;

                let ty_id =
                    self.resolve_ty_id(binary_expr.span(), self.tys().bool, expected_ty_id)?;

                (ty_id, lhs, rhs)
            }
        };

        Ok(BinaryExprAst {
            expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
            bin_op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }
}
