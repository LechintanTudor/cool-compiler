use crate::{BlockExpr, Expr, ParseResult, Parser};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::Span;

#[derive(Clone, Section, Debug)]
pub struct WhileExpr {
    pub span: Span,
    pub condition: Box<Expr>,
    pub body: BlockExpr,
}

impl Parser<'_> {
    pub fn parse_while_expr(&mut self) -> ParseResult<WhileExpr> {
        let while_token = self.bump_expect(&tk::kw_while)?;
        let condition = self.parse_non_struct_expr()?;
        let body = self.parse_block_expr()?;

        Ok(WhileExpr {
            span: while_token.span.to(body.span),
            condition: Box::new(condition),
            body,
        })
    }
}
