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

        let (span, abi) = match as_str_literal(self.peek().kind) {
            Some(abi) => {
                let abi_token = self.bump();
                (extern_token.span.to(abi_token.span), Some(abi))
            }
            None => (extern_token.span, None),
        };

        Ok(FnAbiDecl { span, abi })
    }
}

fn as_str_literal(token_kind: TokenKind) -> Option<Symbol> {
    let TokenKind::Literal(literal) = token_kind else {
        return None;
    };

    if literal.kind != LiteralKind::Str {
        return None;
    };

    Some(literal.value)
}
