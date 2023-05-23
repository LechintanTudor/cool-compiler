use crate::{BlockExpr, CondBlock, ParseResult, Parser};
use cool_lexer::tokens::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct CondExpr {
    pub span: Span,
    pub if_block: Box<CondBlock>,
    pub else_if_blocks: Vec<CondBlock>,
    pub else_block: Option<Box<BlockExpr>>,
}

impl CondExpr {
    #[inline]
    pub fn is_exhaustive(&self) -> bool {
        self.else_block.is_some()
    }
}

impl Section for CondExpr {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_cond_expr(&mut self) -> ParseResult<CondExpr> {
        let start_token = self.bump_expect(&tk::KW_IF)?;
        let if_block = self.parse_cond_block()?;

        let mut else_if_blocks = Vec::<CondBlock>::new();

        loop {
            if self.bump_if_eq(tk::KW_ELSE).is_none() {
                let end_span = else_if_blocks
                    .last()
                    .map(Section::span)
                    .unwrap_or_else(|| if_block.span());

                return Ok(CondExpr {
                    span: start_token.span.to(end_span),
                    if_block: Box::new(if_block),
                    else_if_blocks,
                    else_block: None,
                });
            }

            if self.bump_if_eq(tk::KW_IF).is_none() {
                let else_block = self.parse_block_expr()?;

                return Ok(CondExpr {
                    span: start_token.span.to(else_block.span()),
                    if_block: Box::new(if_block),
                    else_if_blocks,
                    else_block: Some(Box::new(else_block)),
                });
            }

            else_if_blocks.push(self.parse_cond_block()?);
        }
    }
}
