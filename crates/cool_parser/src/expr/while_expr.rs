use crate::{CondBlock, ParseResult, Parser};
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct WhileExpr {
    pub span: Span,
    pub block: Box<CondBlock>,
}

impl Section for WhileExpr {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_while_expr(&mut self) -> ParseResult<WhileExpr> {
        let start_token = self.bump_expect(&tk::KW_WHILE)?;
        let block = self.parse_cond_block()?;

        Ok(WhileExpr {
            span: start_token.span.to(block.span()),
            block: Box::new(block),
        })
    }
}
