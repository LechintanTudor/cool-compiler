use crate::{Ident, ParseResult, Parser, TyId};
use cool_collections::define_index_newtype;
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::{Section, Span};

define_index_newtype!(StructId);

#[derive(Clone, Section, Debug)]
pub struct Struct {
    pub span: Span,
    pub fields: Vec<StructField>,
    pub has_trailing_comma: bool,
}

#[derive(Clone, Section, Debug)]
pub struct StructField {
    pub span: Span,
    pub ident: Ident,
    pub ty: TyId,
}

impl Parser<'_> {
    pub fn parse_struct(&mut self) -> ParseResult<StructId> {
        let struct_token = self.bump_expect(&tk::kw_struct)?;
        self.bump_expect(&tk::open_brace)?;
        let mut fields = Vec::new();

        let (close_brace, has_trailing_comma) =
            if let Some(close_brace) = self.bump_if_eq(tk::close_brace) {
                (close_brace, false)
            } else {
                loop {
                    fields.push(self.parse_struct_field()?);

                    if self.bump_if_eq(tk::comma).is_some() {
                        if let Some(close_brace) = self.bump_if_eq(tk::close_brace) {
                            break (close_brace, true);
                        }
                    } else {
                        break (self.bump_expect(&tk::close_brace)?, false);
                    }
                }
            };

        Ok(self.add_struct(Struct {
            span: struct_token.span.to(close_brace.span),
            fields,
            has_trailing_comma,
        }))
    }

    fn parse_struct_field(&mut self) -> ParseResult<StructField> {
        let ident = self.parse_ident()?;
        self.bump_expect(&tk::colon)?;
        let ty = self.parse_ty()?;

        Ok(StructField {
            span: ident.span.to(self[ty].span()),
            ident,
            ty,
        })
    }
}
