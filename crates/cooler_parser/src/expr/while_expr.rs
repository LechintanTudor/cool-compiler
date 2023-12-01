use crate::{ExprId, ParseResult, Parser};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct WhileExpr {
    pub span: Span,
    pub cond: ExprId,
    pub body: ExprId,
}

impl Parser<'_> {
    pub fn parse_while_expr(&mut self) -> ParseResult<ExprId> {
        let while_token = self.bump_expect(&tk::kw_while)?;
        let cond = self.parse_non_struct_expr()?;
        let body = self.parse_block_expr()?;
        let span = while_token.span.to(self[body].span());
        Ok(self.add_expr(WhileExpr { span, cond, body }))
    }
}
