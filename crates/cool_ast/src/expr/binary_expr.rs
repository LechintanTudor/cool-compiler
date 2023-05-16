use crate::{AstGenerator, AstResult, ExprAst, TyNotComparable};
use cool_parser::{BinOp, BinaryExpr, BitwiseOp};
use cool_resolve::{tys, ExprId, FrameId, ResolveExpr, TyId, TyMismatch};

#[derive(Clone, Debug)]
pub struct BinaryExprAst {
    pub expr_id: ExprId,
    pub bin_op: BinOp,
    pub lhs: Box<ExprAst>,
    pub rhs: Box<ExprAst>,
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
                let lhs_ty_id = self.resolve[lhs.id()].ty_id;
                let rhs = self.gen_expr(frame_id, lhs_ty_id, &binary_expr.rhs)?;

                let ty_id = self
                    .resolve
                    .resolve_direct_ty_id(lhs_ty_id, tys::INFER_NUMBER)?;

                (ty_id, lhs, rhs)
            }
            BinOp::Comparison(_) => {
                let lhs = self.gen_expr(frame_id, tys::INFER, &binary_expr.lhs)?;
                let lhs_ty_id = self.resolve[lhs.id()].ty_id;
                let rhs = self.gen_expr(frame_id, lhs_ty_id, &binary_expr.rhs)?;

                if !self.resolve[lhs_ty_id].is_comparable() {
                    Err(TyNotComparable)?;
                }

                let ty_id = self
                    .resolve
                    .resolve_direct_ty_id(tys::BOOL, expected_ty_id)?;

                (ty_id, lhs, rhs)
            }
            BinOp::Bitwise(bitwise_op) => {
                let lhs = self.gen_expr(frame_id, expected_ty_id, &binary_expr.lhs)?;
                let lhs_ty_id = self.resolve[lhs.id()].ty_id;

                let rhs_expected_ty_id = if lhs_ty_id == tys::BOOL {
                    if matches!(bitwise_op, BitwiseOp::Shl | BitwiseOp::Shr) {
                        Err(TyMismatch {
                            found_ty_id: tys::BOOL,
                            expected_ty_id: tys::INFER_INT,
                        })?;
                    }

                    tys::BOOL
                } else if lhs_ty_id.is_int() {
                    match bitwise_op {
                        BitwiseOp::Shl | BitwiseOp::Shr => tys::INFER_INT,
                        _ => lhs_ty_id,
                    }
                } else {
                    Err(TyMismatch {
                        found_ty_id: lhs_ty_id,
                        expected_ty_id: tys::INFER_INT,
                    })?
                };

                let rhs = self.gen_expr(frame_id, rhs_expected_ty_id, &binary_expr.rhs)?;

                (lhs_ty_id, lhs, rhs)
            }
            BinOp::Logical(_) => {
                let lhs = self.gen_expr(frame_id, tys::BOOL, &binary_expr.lhs)?;
                let rhs = self.gen_expr(frame_id, tys::BOOL, &binary_expr.rhs)?;

                let ty_id = self
                    .resolve
                    .resolve_direct_ty_id(tys::BOOL, expected_ty_id)?;

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
