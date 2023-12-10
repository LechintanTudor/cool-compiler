use crate::{ExprId, Ident, ParseResult, Parser};
use cool_derive::Section;
use cool_lexer::{tk, LiteralKind, TokenKind};
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct AccessExpr {
    pub span: Span,
    pub base: ExprId,
    pub field: Ident,
}

#[derive(Clone, Section, Debug)]
pub struct DerefExpr {
    pub span: Span,
    pub base: ExprId,
}

impl Parser<'_> {
    pub fn continue_parse_access_or_deref_expr(&mut self, base: ExprId) -> ParseResult<ExprId> {
        self.bump_expect(&tk::dot)?;

        let expr = match self.peek().kind {
            TokenKind::Ident(_) => {
                let base_span = self[base].span();
                let field = self.parse_ident()?;

                self.add_expr(AccessExpr {
                    span: base_span.to(field.span),
                    base,
                    field,
                })
            }
            TokenKind::Literal(literal) if literal.kind == LiteralKind::Int => {
                let base_span = self[base].span();
                let field_span = self.bump().span;

                self.add_expr(AccessExpr {
                    span: base_span.to(field_span),
                    base,
                    field: Ident::new(field_span, literal.value),
                })
            }
            tk::star => {
                let base_span = self[base].span();
                let star_span = self.bump().span;

                self.add_expr(DerefExpr {
                    span: base_span.to(star_span),
                    base,
                })
            }
            _ => return self.peek_error(&[tk::identifier, tk::star]),
        };

        Ok(expr)
    }
}
