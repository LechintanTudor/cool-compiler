use crate::{Ident, ParseResult, Parser};
use cool_derive::Section;
use cool_lexer::{tk, LiteralKind as LexerLiteralKind, Symbol, TokenKind};
use cool_span::Span;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LiteralKind {
    Bool,
    Int,
    Float,
    Char { prefix: Option<Symbol> },
    Str { prefix: Option<Symbol> },
}

impl From<LexerLiteralKind> for LiteralKind {
    fn from(kind: LexerLiteralKind) -> Self {
        match kind {
            LexerLiteralKind::Bool => LiteralKind::Bool,
            LexerLiteralKind::Int => LiteralKind::Int,
            LexerLiteralKind::Char => LiteralKind::Char { prefix: None },
            LexerLiteralKind::Str => LiteralKind::Str { prefix: None },
        }
    }
}

#[derive(Clone, Section, Debug)]
pub struct LiteralExpr {
    pub span: Span,
    pub kind: LiteralKind,
    pub value: Symbol,
}

impl Parser<'_> {
    pub fn parse_literal_expr(&mut self) -> ParseResult<LiteralExpr> {
        let start_token = self.bump();
        let TokenKind::Literal(literal) = start_token.kind else {
            return self.error(start_token, &[tk::literal]);
        };

        let float_dot_token = (literal.kind == LexerLiteralKind::Int)
            .then(|| self.bump_any_if_eq(tk::dot))
            .flatten();

        let Some(float_dot_token) = float_dot_token else {
            return Ok(LiteralExpr {
                span: start_token.span,
                kind: literal.kind.into(),
                value: literal.value,
            });
        };

        let next_literal = self
            .peek_any()
            .kind
            .as_literal()
            .filter(|literal| literal.kind == LexerLiteralKind::Int);

        let Some(next_literal) = next_literal else {
            return Ok(LiteralExpr {
                span: start_token.span.to(float_dot_token.span),
                kind: LiteralKind::Float,
                value: literal.value,
            });
        };

        let end_token = self.bump();

        Ok(LiteralExpr {
            span: start_token.span.to(end_token.span),
            kind: LiteralKind::Float,
            value: Symbol::insert(&format!("{}.{}", literal.value, next_literal.value)),
        })
    }

    pub fn continue_parse_literal_expr(&mut self, prefix: Ident) -> ParseResult<LiteralExpr> {
        match self.peek_any().kind {
            TokenKind::Literal(literal) => {
                match literal.kind {
                    LexerLiteralKind::Char => {
                        let end_span = self.bump().span;

                        Ok(LiteralExpr {
                            span: prefix.span.to(end_span),
                            kind: LiteralKind::Char {
                                prefix: Some(prefix.symbol),
                            },
                            value: literal.value,
                        })
                    }
                    LexerLiteralKind::Str => {
                        let end_span = self.bump().span;

                        Ok(LiteralExpr {
                            span: prefix.span.to(end_span),
                            kind: LiteralKind::Str {
                                prefix: Some(prefix.symbol),
                            },
                            value: literal.value,
                        })
                    }
                    _ => self.peek_any_error(&[tk::character, tk::string]),
                }
            }
            _ => self.peek_any_error(&[tk::character, tk::string]),
        }
    }
}
