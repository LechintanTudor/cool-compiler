use crate::{ParseResult, Parser};
use cool_lexer::tokens::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub enum RangeKind {
    Full,
}

#[derive(Clone, Debug)]
pub struct RangeExpr {
    pub span: Span,
    pub kind: RangeKind,
}

impl Section for RangeExpr {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_range_expr(&mut self) -> ParseResult<RangeExpr> {
        let range = self.bump_expect(&tk::DOT_DOT)?;

        Ok(RangeExpr {
            span: range.span,
            kind: RangeKind::Full,
        })
    }
}
