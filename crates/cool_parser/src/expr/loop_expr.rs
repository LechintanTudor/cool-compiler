use crate::{ExprId, ParseResult, Parser};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct LoopExpr {
    pub span: Span,
    pub body: ExprId,
}

impl Parser<'_> {
    pub fn parse_loop_expr(&mut self) -> ParseResult<ExprId> {
        let loop_token = self.bump_expect(&tk::kw_loop)?;
        let body = self.parse_block_expr()?;
        let span = loop_token.span.to(self[body].span());
        Ok(self.add_expr(LoopExpr { span, body }))
    }
}
