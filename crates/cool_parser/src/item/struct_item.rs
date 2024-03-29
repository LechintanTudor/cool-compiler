use crate::{Ident, ParseResult, Parser, Ty};
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct StructField {
    pub ident: Ident,
    pub ty: Ty,
}

impl Section for StructField {
    #[inline]
    fn span(&self) -> Span {
        self.ident.span().to(self.ty.span())
    }
}

#[derive(Clone, Debug)]
pub struct StructItem {
    pub span: Span,
    pub has_body: bool,
    pub fields: Vec<StructField>,
    pub has_trailing_comma: bool,
}

impl Section for StructItem {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_struct_field(&mut self) -> ParseResult<StructField> {
        let ident = self.parse_ident()?;
        self.bump_expect(&tk::COLON)?;
        let ty = self.parse_ty()?;

        Ok(StructField { ident, ty })
    }

    pub fn parse_struct_item(&mut self) -> ParseResult<StructItem> {
        let start_token = self.bump_expect(&tk::KW_STRUCT)?;

        if self.peek().kind != tk::OPEN_BRACE {
            return Ok(StructItem {
                span: start_token.span,
                has_body: false,
                fields: vec![],
                has_trailing_comma: false,
            });
        }

        self.bump_expect(&tk::OPEN_BRACE)?;

        let mut fields = Vec::<StructField>::new();
        let (end_token, has_trailing_comma) = if self.peek().kind == tk::CLOSE_BRACE {
            (self.bump(), false)
        } else {
            loop {
                fields.push(self.parse_struct_field()?);

                let next_token = self.bump();
                match next_token.kind {
                    tk::COMMA => {
                        if let Some(end_token) = self.bump_if_eq(tk::CLOSE_BRACE) {
                            break (end_token, true);
                        }
                    }
                    tk::CLOSE_BRACE => {
                        break (next_token, false);
                    }
                    _ => self.error(next_token, &[tk::COMMA, tk::CLOSE_BRACE])?,
                }
            }
        };

        Ok(StructItem {
            span: start_token.span.to(end_token.span),
            has_body: true,
            fields,
            has_trailing_comma,
        })
    }
}
