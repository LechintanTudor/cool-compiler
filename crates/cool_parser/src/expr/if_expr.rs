use crate::{ExprId, ParseResult, Parser};
use cool_collections::smallvec::smallvec;
use cool_collections::SmallVec;
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct IfExpr {
    pub span: Span,
    pub cond_blocks: SmallVec<CondBlock, 2>,
    pub else_block: Option<ExprId>,
}

#[derive(Clone, Debug)]
pub struct CondBlock {
    pub cond: ExprId,
    pub body: ExprId,
}

impl Parser<'_> {
    pub fn parse_if_expr(&mut self) -> ParseResult<ExprId> {
        let if_token = self.bump_expect(&tk::kw_if)?;
        let mut cond_blocks: SmallVec<CondBlock, 2> = smallvec![self.parse_cond_block()?];
        let mut else_block = Option::<ExprId>::None;

        while self.bump_if_eq(tk::kw_else).is_some() {
            if self.bump_if_eq(tk::kw_if).is_some() {
                cond_blocks.push(self.parse_cond_block()?);
            } else {
                else_block = Some(self.parse_block_expr()?);
                break;
            }
        }

        let end_span = else_block
            .map(|expr| self[expr].span())
            .unwrap_or_else(|| self[cond_blocks.last().unwrap().body].span());

        Ok(self.add_expr(IfExpr {
            span: if_token.span.to(end_span),
            cond_blocks,
            else_block,
        }))
    }

    fn parse_cond_block(&mut self) -> ParseResult<CondBlock> {
        Ok(CondBlock {
            cond: self.parse_non_struct_expr()?,
            body: self.parse_block_expr()?,
        })
    }
}
