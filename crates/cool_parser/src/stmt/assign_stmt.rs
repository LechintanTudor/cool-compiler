use crate::expr::Expr;
use crate::{ParseResult, ParseTree, Parser, Pattern};
use cool_lexer::tokens::{tk, Token};
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct AssignStmt {
    pub span: Span,
    pub pattern: Pattern,
    pub expr: Expr,
}

impl ParseTree for AssignStmt {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn continue_parse_assign_after_pattern(
        &mut self,
        pattern: Pattern,
    ) -> ParseResult<AssignStmt> {
        self.bump_expect(&[tk::EQ])?;
        let expr = self.parse_expr()?;

        Ok(AssignStmt {
            span: pattern.span_to(&expr),
            pattern,
            expr,
        })
    }
}
