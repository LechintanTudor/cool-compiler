use crate::{ExprId, Ident, ParseResult, Parser};
use cool_collections::SmallVec;
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct StructExpr {
    pub span: Span,
    pub base: ExprId,
    pub fields: SmallVec<StructExprField, 1>,
    pub has_trailing_comma: bool,
    pub fill: Option<ExprId>,
}

#[derive(Clone, Debug)]
pub struct StructExprField {
    pub ident: Ident,
    pub value: Option<ExprId>,
}

impl Parser<'_> {
    pub fn continue_parse_struct_expr(&mut self, base: ExprId) -> ParseResult<ExprId> {
        self.bump_expect(&tk::open_brace)?;
        let mut fields = SmallVec::new();

        let (close_brace, has_trailing_comma, fill) =
            if let Some(close_brace) = self.bump_if_eq(tk::close_brace) {
                (close_brace, false, None)
            } else {
                loop {
                    if self.bump_if_eq(tk::dot_dot).is_some() {
                        let fill = self.parse_expr()?;
                        let close_brace = self.bump_expect(&tk::close_brace)?;
                        break (close_brace, false, Some(fill));
                    }

                    fields.push(self.parse_struct_expr_field()?);

                    if self.bump_if_eq(tk::comma).is_some() {
                        if let Some(close_brace) = self.bump_if_eq(tk::close_brace) {
                            break (close_brace, true, None);
                        }
                    } else {
                        let close_brace = self.bump_expect(&tk::close_brace)?;
                        break (close_brace, false, None);
                    }
                }
            };

        let start_span = self[base].span();

        Ok(self.add_expr(StructExpr {
            span: start_span.to(close_brace.span),
            base,
            fields,
            has_trailing_comma,
            fill,
        }))
    }

    fn parse_struct_expr_field(&mut self) -> ParseResult<StructExprField> {
        let ident = self.parse_ident()?;

        let value = self
            .bump_if_eq(tk::eq)
            .map(|_| self.parse_expr())
            .transpose()?;

        Ok(StructExprField { ident, value })
    }
}
