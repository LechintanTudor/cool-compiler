use crate::{Ident, ParseResult, Parser, Ty};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct StructItem {
    pub span: Span,
    pub fields: Vec<StructItemField>,
    pub has_trailing_comma: bool,
}

#[derive(Clone, Debug)]
pub struct StructItemField {
    pub ident: Ident,
    pub ty: Ty,
}

impl Section for StructItemField {
    #[inline]
    fn span(&self) -> Span {
        self.ident.span.to(self.ty.span())
    }
}

impl Parser<'_> {
    pub fn parse_struct_item(&mut self) -> ParseResult<StructItem> {
        let struct_token = self.bump_expect(&tk::kw_struct)?;
        self.bump_expect(&tk::open_brace)?;

        let mut fields = Vec::<StructItemField>::new();

        let (close_brace, has_trailing_comma) = match self.bump_if_eq(tk::close_brace) {
            Some(close_brace) => (close_brace, false),
            None => {
                loop {
                    fields.push(self.parse_struct_item_field()?);

                    let next_token = self.bump();

                    match next_token.kind {
                        tk::close_brace => {
                            break (next_token, false);
                        }
                        tk::comma => {
                            if let Some(close_brace) = self.bump_if_eq(tk::close_brace) {
                                break (close_brace, true);
                            }
                        }
                        _ => return self.error(next_token, &[tk::comma, tk::close_brace]),
                    }
                }
            }
        };

        Ok(StructItem {
            span: struct_token.span.to(close_brace.span),
            fields,
            has_trailing_comma,
        })
    }

    fn parse_struct_item_field(&mut self) -> ParseResult<StructItemField> {
        let ident = self.parse_ident()?;
        self.bump_expect(&tk::colon)?;
        let ty = self.parse_ty()?;

        Ok(StructItemField { ident, ty })
    }
}
