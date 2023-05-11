use crate::expr::Expr;
use crate::{AssignOp, ParseResult, Parser};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct AssignStmt {
    pub assign_op: AssignOp,
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

impl Section for AssignStmt {
    #[inline]
    fn span(&self) -> Span {
        self.lhs.span().to(self.rhs.span())
    }
}

impl Parser<'_> {
    pub fn continue_parse_assign(
        &mut self,
        lhs: Expr,
        assign_op: AssignOp,
    ) -> ParseResult<AssignStmt> {
        let rhs = self.parse_expr()?;

        Ok(AssignStmt {
            assign_op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }
}
