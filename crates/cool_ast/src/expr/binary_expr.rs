use crate::{BinaryOp, ExprId, ParseResult, Parser};
use cool_collections::smallvec::smallvec;
use cool_collections::SmallVec;
use cool_derive::Section;
use cool_span::{Section, Span};
use derive_more::From;

#[derive(Clone, Section, Debug)]
pub struct BinaryExpr {
    pub span: Span,
    pub lhs: ExprId,
    pub op: BinaryOp,
    pub rhs: ExprId,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, From, Debug)]
enum ExprPart {
    Expr(ExprId),
    BinaryOp(BinaryOp),
}

impl ExprPart {
    #[inline]
    #[must_use]
    fn unwrap_expr(self) -> ExprId {
        let Self::Expr(expr_id) = self else {
            panic!("Part is not an expression");
        };

        expr_id
    }
}

impl Parser<'_> {
    pub fn parse_expr_full(&mut self, allow_struct: bool) -> ParseResult<ExprId> {
        let expr = self.parse_primary_expr(allow_struct)?;

        let (first_binary_op, second_expr) = match BinaryOp::try_from(self.peek().kind) {
            Ok(binary_op) => {
                self.bump();
                (binary_op, self.parse_primary_expr(allow_struct)?)
            }
            Err(_) => return Ok(expr),
        };

        let mut parts: SmallVec<ExprPart, 8> = smallvec![expr.into(), second_expr.into()];
        let mut binary_ops: SmallVec<BinaryOp, 8> = smallvec![first_binary_op];

        while let Ok(binary_op) = BinaryOp::try_from(self.peek().kind) {
            self.bump();

            while let Some(&last_binary_op) = binary_ops.last() {
                if last_binary_op.precedence() < binary_op.precedence() {
                    break;
                }

                parts.push(last_binary_op.into());
                binary_ops.pop();
            }

            binary_ops.push(binary_op);
            parts.push(self.parse_primary_expr(allow_struct)?.into());
        }

        while let Some(binary_op) = binary_ops.pop() {
            parts.push(binary_op.into());
        }

        let mut part_stack = SmallVec::<ExprPart, 8>::new();

        for part in parts.drain(..) {
            match part {
                ExprPart::Expr(_) => {
                    part_stack.push(part);
                }
                ExprPart::BinaryOp(binary_op) => {
                    let rhs = part_stack.pop().unwrap().unwrap_expr();
                    let lhs = part_stack.pop().unwrap().unwrap_expr();
                    let span = self[lhs].span().to(self[rhs].span());

                    part_stack.push(
                        self.add_expr(BinaryExpr {
                            span,
                            lhs,
                            op: binary_op,
                            rhs,
                        })
                        .into(),
                    );
                }
            }
        }

        debug_assert!(part_stack.len() == 1);
        Ok(part_stack.pop().unwrap().unwrap_expr())
    }
}
