use crate::{BlockExpr, ParseResult, Parser};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::Span;

#[derive(Clone, Section, Debug)]
pub struct LoopExpr {
    pub span: Span,
    pub body: BlockExpr,
}

impl Parser<'_> {
    pub fn parse_loop_expr(&mut self) -> ParseResult<LoopExpr> {
        let loop_token = self.bump_expect(&tk::kw_loop)?;
        let body = self.parse_block_expr()?;

        Ok(LoopExpr {
            span: loop_token.span.to(body.span),
            body,
        })
    }
}
