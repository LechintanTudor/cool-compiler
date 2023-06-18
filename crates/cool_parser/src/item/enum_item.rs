use crate::{Ident, ParseResult, Parser, Ty};
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct EnumStorage {
    pub span: Span,
    pub ty: Box<Ty>,
    pub has_trailing_comma: bool,
}

impl Section for EnumStorage {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone, Debug)]
pub struct EnumItem {
    pub span: Span,
    pub storage: Option<EnumStorage>,
    pub variants: Vec<Ident>,
    pub has_trailing_comma: bool,
}

impl Section for EnumItem {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_enum_item(&mut self) -> ParseResult<EnumItem> {
        let start_token = self.bump_expect(&tk::KW_ENUM)?;

        let storage = (self.peek().kind == tk::OPEN_PAREN)
            .then(|| self.parse_enum_storage())
            .transpose()?;

        if let Some(end_token) = self.bump_if_eq(tk::CLOSE_PAREN) {
            return Ok(EnumItem {
                span: start_token.span.to(end_token.span),
                storage,
                variants: vec![],
                has_trailing_comma: false,
            });
        }

        let mut variants = Vec::<Ident>::new();

        let (end_token, has_trailing_comma) = loop {
            variants.push(self.parse_ident()?);

            match self.bump_if_eq(tk::CLOSE_PAREN) {
                Some(end_token) => break (end_token, false),
                None => {
                    self.bump_expect(&tk::COMMA)?;

                    if let Some(end_token) = self.bump_if_eq(tk::CLOSE_PAREN) {
                        break (end_token, true);
                    }
                }
            }
        };

        Ok(EnumItem {
            span: start_token.span.to(end_token.span),
            storage,
            variants,
            has_trailing_comma,
        })
    }

    fn parse_enum_storage(&mut self) -> ParseResult<EnumStorage> {
        let open_brace = self.bump_expect(&tk::OPEN_PAREN)?;
        let ty = self.parse_ty()?;
        let has_trailing_comma = self.bump_if_eq(tk::COMMA).is_some();
        let close_paren = self.bump_expect(&tk::CLOSE_PAREN)?;

        Ok(EnumStorage {
            span: open_brace.span.to(close_paren.span),
            ty: Box::new(ty),
            has_trailing_comma,
        })
    }
}
