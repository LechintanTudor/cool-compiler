use crate::{Expr, Ident, ParseResult, Parser};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct StructExpr {
    pub span: Span,
    pub base: Box<Expr>,
    pub fields: Vec<StructExprFieldInitializer>,
    pub has_trailing_comma: bool,
    pub update: Option<Box<Expr>>,
}

#[derive(Clone, Debug)]
pub struct StructExprFieldInitializer {
    pub field: Ident,
    pub value: Option<Expr>,
}

impl Section for StructExprFieldInitializer {
    #[inline]
    fn span(&self) -> Span {
        let end_span = self
            .value
            .as_ref()
            .map(|expr| expr.span())
            .unwrap_or(self.field.span);

        self.field.span.to(end_span)
    }
}

impl Parser<'_> {
    pub fn continue_parse_struct_expr(&mut self, base: Expr) -> ParseResult<StructExpr> {
        self.bump_expect(&tk::open_brace)?;
        let mut fields = Vec::<StructExprFieldInitializer>::new();

        let (close_brace, has_trailing_comma, update) =
            if let Some(close_brace) = self.bump_if_eq(tk::close_brace) {
                (close_brace, false, None)
            } else {
                loop {
                    if self.bump_if_eq(tk::dot_dot).is_some() {
                        let update = self.parse_expr()?;
                        let close_brace = self.bump_expect(&tk::close_brace)?;
                        break (close_brace, false, Some(Box::new(update)));
                    }

                    fields.push(self.parse_struct_expr_field_initializer()?);

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

        Ok(StructExpr {
            span: base.span().to(close_brace.span),
            base: Box::new(base),
            fields,
            has_trailing_comma,
            update,
        })
    }

    fn parse_struct_expr_field_initializer(&mut self) -> ParseResult<StructExprFieldInitializer> {
        let field = self.parse_ident()?;

        let value = self
            .bump_if_eq(tk::eq)
            .map(|_| self.parse_expr())
            .transpose()?;

        Ok(StructExprFieldInitializer { field, value })
    }
}
