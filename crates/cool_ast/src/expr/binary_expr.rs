use crate::{AstGenerator, AstResult, ExprAst, TyMismatch, TyNotComparable};
use cool_parser::{BinOp, BinaryExpr, BitwiseOp};
use cool_resolve::{tys, ExprId, FrameId, TyId};

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
                let ty_id = self.resolve[lhs.id()].ty_id;
                let rhs = self.gen_expr(frame_id, ty_id, &binary_expr.rhs)?;

                if !ty_id.is_number() {
                    Err(TyMismatch {
                        found: ty_id,
                        expected: tys::INFERRED_NUMBER,
                    })?;
                }

                (ty_id, lhs, rhs)
            }
            BinOp::Comparison(_) => {
                let lhs = self.gen_expr(frame_id, tys::INFERRED, &binary_expr.lhs)?;
                let lhs_ty_id = self.resolve[lhs.id()].ty_id;
                let rhs = self.gen_expr(frame_id, lhs_ty_id, &binary_expr.rhs)?;

                if !self.resolve[lhs_ty_id].is_comparable() {
                    Err(TyNotComparable)?;
                }

                let ty_id = tys::BOOL
                    .resolve_non_inferred(expected_ty_id)
                    .ok_or(TyMismatch {
                        found: tys::BOOL,
                        expected: expected_ty_id,
                    })?;

                (ty_id, lhs, rhs)
            }
            BinOp::Bitwise(bitwise_op) => {
                let lhs = self.gen_expr(frame_id, expected_ty_id, &binary_expr.lhs)?;
                let lhs_ty_id = self.resolve[lhs.id()].ty_id;

                if !lhs_ty_id.is_int() {
                    Err(TyMismatch {
                        found: lhs_ty_id,
                        expected: tys::INFERRED_INT,
                    })?;
                }

                let rhs_expected_ty_id = match bitwise_op {
                    BitwiseOp::Shl | BitwiseOp::Shr => tys::INFERRED_INT,
                    _ => lhs_ty_id,
                };

                let rhs = self.gen_expr(frame_id, rhs_expected_ty_id, &binary_expr.rhs)?;

                (lhs_ty_id, lhs, rhs)
            }
            BinOp::Logical(_) => {
                let lhs = self.gen_expr(frame_id, tys::BOOL, &binary_expr.lhs)?;
                let rhs = self.gen_expr(frame_id, tys::BOOL, &binary_expr.rhs)?;

                let ty_id = tys::BOOL
                    .resolve_non_inferred(expected_ty_id)
                    .ok_or(TyMismatch {
                        found: tys::BOOL,
                        expected: expected_ty_id,
                    })?;

                (ty_id, lhs, rhs)
            }
        };

        Ok(BinaryExprAst {
            expr_id: self.resolve.add_expr(ty_id, false),
            bin_op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }
}
