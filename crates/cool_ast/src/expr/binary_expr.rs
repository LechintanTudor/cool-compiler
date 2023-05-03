use crate::{ArithmeticBinOpAst, AstGenerator, AstResult, BinOpAst, ExprAst, NumberKind};
use cool_parser::{BinOp, BinaryExpr};
use cool_resolve::{ExprId, FrameId, TyId};

#[derive(Clone, Debug)]
pub struct BinaryExprAst {
    pub expr_id: ExprId,
    pub bin_op: BinOpAst,
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
        let expr = match binary_expr.bin_op {
            BinOp::Arithmetic(bin_op) => {
                let lhs = self.gen_expr(frame_id, expected_ty_id, &binary_expr.lhs)?;
                let ty_id = self.resolve[lhs.id()];
                let rhs = self.gen_expr(frame_id, ty_id, &binary_expr.rhs)?;

                let number_kind = NumberKind::try_from(ty_id)?;
                let bin_op = ArithmeticBinOpAst::new(bin_op, number_kind);

                BinaryExprAst {
                    expr_id: self.resolve.add_expr(ty_id),
                    bin_op: bin_op.into(),
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                }
            }
            _ => todo!(),
        };

        Ok(expr)
    }
}
