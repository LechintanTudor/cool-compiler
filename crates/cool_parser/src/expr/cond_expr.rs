use crate::{BlockExpr, Expr, ParseResult, ParseTree, Parser};
use cool_lexer::tokens::tk;
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct CondBlock {
    pub cond: Box<Expr>,
    pub expr: BlockExpr,
}

impl ParseTree for CondBlock {
    #[inline]
    fn span(&self) -> Span {
        self.cond.span().to(self.expr.span())
    }
}

#[derive(Clone, Debug)]
pub struct CondExpr {
    pub span: Span,
    pub if_block: CondBlock,
    pub else_if_blocks: Vec<CondBlock>,
    pub else_block: Option<BlockExpr>,
}

impl ParseTree for CondExpr {
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
                    .map(ParseTree::span)
                    .unwrap_or_else(|| if_block.span());

                return Ok(CondExpr {
                    span: start_token.span.to(end_span),
                    if_block,
                    else_if_blocks: vec![],
                    else_block: None,
                });
            }

            if self.bump_if_eq(tk::KW_IF).is_none() {
                let else_block = self.parse_block_expr()?;

                return Ok(CondExpr {
                    span: start_token.span.to(else_block.span()),
                    if_block,
                    else_if_blocks: vec![],
                    else_block: Some(else_block),
                });
            }

            else_if_blocks.push(self.parse_cond_block()?);
        }
    }

    fn parse_cond_block(&mut self) -> ParseResult<CondBlock> {
        let cond = self.parse_guard_expr()?;
        let expr = self.parse_block_expr()?;

        Ok(CondBlock {
            cond: Box::new(cond),
            expr,
        })
    }
}
