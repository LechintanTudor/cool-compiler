use crate::{BlockExpr, Expr, ParseResult, Parser};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::Span;

#[derive(Clone, Section, Debug)]
pub struct IfExpr {
    pub span: Span,
    pub cond_exprs: Vec<(Expr, BlockExpr)>,
    pub else_expr: Option<Box<BlockExpr>>,
}

impl Parser<'_> {
    pub fn parse_if_expr(&mut self) -> ParseResult<IfExpr> {
        let if_token = self.bump_expect(&tk::kw_if)?;

        let first_cond = self.parse_non_struct_expr()?;
        let first_expr = self.parse_block_expr()?;
        let mut cond_exprs = vec![(first_cond, first_expr)];

        let else_expr = loop {
            if self.bump_if_eq(tk::kw_else).is_none() {
                break None;
            };

            if self.bump_if_eq(tk::kw_if).is_some() {
                let cond = self.parse_non_struct_expr()?;
                let expr = self.parse_block_expr()?;
                cond_exprs.push((cond, expr));
            } else {
                break Some(Box::new(self.parse_block_expr()?));
            }
        };

        let end_span = else_expr
            .as_ref()
            .map(|expr| expr.span)
            .unwrap_or(cond_exprs.last().unwrap().1.span);

        Ok(IfExpr {
            span: if_token.span.to(end_span),
            cond_exprs,
            else_expr,
        })
    }
}
