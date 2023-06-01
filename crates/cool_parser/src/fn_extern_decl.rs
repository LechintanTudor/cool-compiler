use crate::{ParseResult, Parser};
use cool_lexer::{tk, Literal, LiteralKind, Symbol, TokenKind};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct FnExternDecl {
    pub span: Span,
    pub abi: Option<Symbol>,
}

impl Section for FnExternDecl {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_fn_extern_decl(&mut self) -> ParseResult<FnExternDecl> {
        let start_token = self.bump_expect(&tk::KW_EXTERN)?;

        let (end_span, abi) = match self.peek().kind {
            TokenKind::Literal(Literal {
                kind: LiteralKind::Str,
                symbol,
            }) => (self.bump().span, Some(symbol)),
            _ => (start_token.span, None),
        };

        Ok(FnExternDecl {
            span: start_token.span.to(end_span),
            abi,
        })
    }
}
