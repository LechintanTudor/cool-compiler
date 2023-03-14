use crate::{BlockElem, ParseResult, ParseTree, Parser};
use cool_lexer::tokens::tk;
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct BlockExpr {
    pub span: Span,
    pub elems: Vec<BlockElem>,
}

impl ParseTree for BlockExpr {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_block_expr(&mut self) -> ParseResult<BlockExpr> {
        let start_token = self.bump_expect(&[tk::OPEN_BRACE])?;

        let mut elems = Vec::<BlockElem>::new();
        while self.peek().kind != tk::CLOSE_BRACE {
            let elem = self.parse_block_elem()?;
            let is_expr = matches!(elem, BlockElem::Expr(_));
            elems.push(elem);

            if is_expr {
                break;
            }
        }

        let end_token = self.bump_expect(&[tk::CLOSE_BRACE])?;

        Ok(BlockExpr {
            span: start_token.span.to(end_token.span),
            elems,
        })
    }
}
