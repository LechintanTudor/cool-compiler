use crate::{ParseResult, ParseTree, Parser, UnexpectedToken};
use cool_lexer::symbols::Symbol;
use cool_lexer::tokens::{tk, Literal, TokenKind};
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct LiteralExpr {
    pub span: Span,
    pub prefix: Option<Symbol>,
    pub literal: Literal,
}

impl ParseTree for LiteralExpr {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_literal_expr(&mut self) -> ParseResult<LiteralExpr> {
        let start_token = self.bump();

        let (prefix, literal, end_token) = match start_token.kind {
            TokenKind::Prefix(symbol) => {
                let next_token = self.bump();
                let TokenKind::Literal(literal) = next_token.kind else {
                    Err(UnexpectedToken {
                        found: next_token,
                        expected: &[tk::ANY_LITERAL],
                    })?
                };

                (Some(symbol), literal, next_token)
            }
            TokenKind::Literal(literal) => (None, literal, start_token),
            _ => Err(UnexpectedToken {
                found: start_token,
                expected: &[tk::ANY_IDENT, tk::ANY_LITERAL],
            })?,
        };

        Ok(LiteralExpr {
            span: start_token.span.to(end_token.span),
            prefix,
            literal,
        })
    }
}
