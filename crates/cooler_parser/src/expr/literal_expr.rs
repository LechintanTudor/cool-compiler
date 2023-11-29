use crate::{parse_error, ExprId, Ident, ParseResult, Parser};
use cool_collections::SmallString;
use cool_derive::Section;
use cool_lexer::{tk, LiteralKind, Symbol, TokenKind};
use cool_span::Span;
use std::fmt::Write;

#[derive(Clone, Section, Debug)]
pub struct LiteralExpr {
    pub span: Span,
    pub kind: LiteralExprKind,
    pub value: Symbol,
}

#[derive(Clone, Debug)]
pub enum LiteralExprKind {
    Bool,
    Int,
    Float,
    Char { prefix: Option<Symbol> },
    Str { prefix: Option<Symbol> },
}

impl From<LiteralKind> for LiteralExprKind {
    #[inline]
    fn from(kind: LiteralKind) -> Self {
        match kind {
            LiteralKind::Bool => Self::Bool,
            LiteralKind::Int => Self::Int,
            LiteralKind::Char => Self::Char { prefix: None },
            LiteralKind::Str => Self::Str { prefix: None },
        }
    }
}

impl Parser<'_> {
    pub fn parse_literal_expr(&mut self) -> ParseResult<ExprId> {
        let token = self.bump();
        let TokenKind::Literal(literal) = token.kind else {
            return parse_error(token, &[tk::literal]);
        };

        let dot_token = (literal.kind == LiteralKind::Int)
            .then(|| self.bump_any_if_eq(tk::dot))
            .flatten();

        let Some(dot_token) = dot_token else {
            return Ok(self.data.exprs.push(
                LiteralExpr {
                    span: token.span,
                    kind: literal.kind.into(),
                    value: literal.value,
                }
                .into(),
            ));
        };

        let (span, value) = self
            .try_parse_decimal_part()
            .map(|(decimal_span, decimal)| {
                let mut value: SmallString = SmallString::new();
                write!(&mut value, "{}.{}", literal.value, decimal).unwrap();
                (token.span.to(decimal_span), Symbol::insert(&value))
            })
            .unwrap_or_else(|| {
                let mut value: SmallString = SmallString::new();
                write!(&mut value, "{}.", literal.value).unwrap();
                (token.span.to(dot_token.span), Symbol::insert(&value))
            });

        Ok(self.data.exprs.push(
            LiteralExpr {
                span,
                kind: literal.kind.into(),
                value,
            }
            .into(),
        ))
    }

    pub fn continue_parse_literal_expr(&mut self, prefix: Ident) -> ParseResult<ExprId> {
        let token = self.bump_any();

        let (kind, value) = match token.kind {
            TokenKind::Literal(literal) => {
                let prefix = Some(prefix.symbol);

                let kind = match literal.kind {
                    LiteralKind::Char => LiteralExprKind::Char { prefix },
                    LiteralKind::Str => LiteralExprKind::Str { prefix },
                    _ => return parse_error(token, &[tk::literal]),
                };

                (kind, literal.value)
            }
            _ => return parse_error(token, &[tk::literal]),
        };

        Ok(self.data.exprs.push(
            LiteralExpr {
                span: prefix.span.to(token.span),
                kind,
                value,
            }
            .into(),
        ))
    }

    fn try_parse_decimal_part(&mut self) -> Option<(Span, Symbol)> {
        match self.peek_any().kind {
            TokenKind::Literal(literal) if literal.kind == LiteralKind::Int => {
                let span = self.bump().span;
                Some((span, literal.value))
            }
            _ => None,
        }
    }
}
