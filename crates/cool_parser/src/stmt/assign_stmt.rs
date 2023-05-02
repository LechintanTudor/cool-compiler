use crate::expr::Expr;
use crate::{AssignOp, ParseResult, ParseTree, Parser};
use cool_lexer::tokens::tk;
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct AssignStmt {
    pub span: Span,
    pub assign_op: AssignOp,
    pub lvalue: Expr,
    pub rvalue: Expr,
}

impl ParseTree for AssignStmt {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn continue_parse_assign_after_assign_op(
        &mut self,
        lvalue: Expr,
        assign_op: AssignOp,
    ) -> ParseResult<AssignStmt> {
        let rvalue = self.parse_expr()?;
        let end_token = self.bump_expect(&tk::SEMICOLON)?;

        Ok(AssignStmt {
            span: lvalue.span().to(end_token.span),
            assign_op,
            lvalue,
            rvalue,
        })
    }
}
