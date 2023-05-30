use crate::{LiteralExpr, ParseResult, Parser, Ty};
use cool_lexer::tokens::{tk, Token};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct ArrayTy {
    pub span: Span,
    pub len: LiteralExpr,
    pub elem: Box<Ty>,
}

impl Section for ArrayTy {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub(crate) fn continue_parse_array_ty(&mut self, open_bracket: Token) -> ParseResult<ArrayTy> {
        debug_assert_eq!(open_bracket.kind, tk::OPEN_BRACKET);

        let len = self.parse_literal_expr()?;
        self.bump_expect(&tk::CLOSE_BRACKET)?;
        let elem = self.parse_ty()?;

        Ok(ArrayTy {
            span: open_bracket.span.to(elem.span()),
            len,
            elem: Box::new(elem),
        })
    }
}
