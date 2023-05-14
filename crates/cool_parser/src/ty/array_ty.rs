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
    pub(crate) fn continue_parse_array_ty(&mut self, start_token: Token) -> ParseResult<ArrayTy> {
        let len = self.parse_literal_expr()?;
        self.bump_expect(&tk::CLOSE_BRACKET)?;
        let elem = self.parse_ty()?;

        Ok(ArrayTy {
            span: start_token.span.to(elem.span()),
            len,
            elem: Box::new(elem),
        })
    }
}
