use crate::{Expr, Ident, ParseResult, Parser};
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct StructFieldInitializer {
    pub ident: Ident,
    pub expr: Box<Expr>,
}

impl Section for StructFieldInitializer {
    #[inline]
    fn span(&self) -> Span {
        self.ident.span.to(self.expr.span())
    }
}

#[derive(Clone, Debug)]
pub struct StructExpr {
    pub span: Span,
    pub base: Box<Expr>,
    pub initializers: Vec<StructFieldInitializer>,
    pub has_trailing_comma: bool,
}

impl Section for StructExpr {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_struct_field_initializer(&mut self) -> ParseResult<StructFieldInitializer> {
        let ident = self.parse_ident()?;
        self.bump_expect(&tk::EQ)?;
        let expr = self.parse_expr()?;

        Ok(StructFieldInitializer {
            ident,
            expr: Box::new(expr),
        })
    }

    pub fn continue_parse_struct_expr(&mut self, base: Box<Expr>) -> ParseResult<StructExpr> {
        self.bump_expect(&tk::OPEN_BRACE)?;

        if let Some(end_token) = self.bump_if_eq(tk::CLOSE_BRACE) {
            return Ok(StructExpr {
                span: base.span().to(end_token.span),
                base,
                initializers: Default::default(),
                has_trailing_comma: false,
            });
        }

        let mut initializers = Vec::<StructFieldInitializer>::new();

        let (end_token, has_trailing_comma) = loop {
            initializers.push(self.parse_struct_field_initializer()?);

            if let Some(end_token) = self.bump_if_eq(tk::CLOSE_BRACE) {
                break (end_token, false);
            } else if self.bump_if_eq(tk::COMMA).is_some() {
                if let Some(end_token) = self.bump_if_eq(tk::CLOSE_BRACE) {
                    break (end_token, true);
                }
            } else {
                self.peek_error(&[tk::CLOSE_BRACE, tk::COMMA])?;
            }
        };

        Ok(StructExpr {
            span: base.span().to(end_token.span),
            base,
            initializers,
            has_trailing_comma,
        })
    }
}
