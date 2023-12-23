use crate::{ParseResult, Parser};
use cool_derive::Section;
use cool_lexer::{tk, LiteralKind, Symbol, TokenKind};
use cool_span::Span;

#[derive(Clone, Section, Debug)]
pub struct FnAbiDecl {
    pub span: Span,
    pub abi: Option<Symbol>,
}

impl Parser<'_> {
    pub fn parse_fn_abi_decl(&mut self) -> ParseResult<FnAbiDecl> {
        let extern_token = self.bump_expect(&tk::kw_extern)?;

        let (span, abi) = match self.parse_str_literal() {
            Some((end_span, symbol)) => (extern_token.span.to(end_span), Some(symbol)),
            None => (extern_token.span, None),
        };

        Ok(FnAbiDecl { span, abi })
    }

    fn parse_str_literal(&mut self) -> Option<(Span, Symbol)> {
        let token = self.peek();

        let TokenKind::Literal(literal) = token.kind else {
            return None;
        };

        if literal.kind != LiteralKind::Str {
            return None;
        };

        self.bump();
        Some((token.span, literal.value))
    }
}
