use crate::{BlockExpr, ParseResult, Parser};
use cool_lexer::tokens::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct LoopExpr {
    pub span: Span,
    pub block: Box<BlockExpr>,
}

impl Section for LoopExpr {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_loop_expr(&mut self) -> ParseResult<LoopExpr> {
        let start_token = self.bump_expect(&tk::KW_LOOP)?;
        let block = self.parse_block_expr()?;

        Ok(LoopExpr {
            span: start_token.span.to(block.span()),
            block: Box::new(block),
        })
    }
}
