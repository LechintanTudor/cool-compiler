use crate::{AssignOp, Expr, ParseResult, Parser};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct AssignStmt {
    pub lhs: Box<Expr>,
    pub op: AssignOp,
    pub rhs: Box<Expr>,
}

impl Section for AssignStmt {
    #[inline]
    fn span(&self) -> Span {
        self.lhs.span().to(self.rhs.span())
    }
}

impl Parser<'_> {
    pub fn continue_parse_assign_stmt(
        &mut self,
        lhs: Expr,
        op: AssignOp,
    ) -> ParseResult<AssignStmt> {
        let rhs = self.parse_expr()?;

        Ok(AssignStmt {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
        })
    }
}
