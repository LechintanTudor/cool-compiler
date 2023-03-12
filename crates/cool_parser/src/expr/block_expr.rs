use crate::{ParseResult, ParseTree, Parser};
use cool_lexer::tokens::Token;
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct BlockExpr {
    pub span: Span,
}

impl ParseTree for BlockExpr {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn parse_block_expr(&mut self) -> ParseResult<BlockExpr> {
        todo!()
    }
}
