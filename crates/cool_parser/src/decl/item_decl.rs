use crate::{Ident, Item, ParseResult, Parser, Ty};
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct ItemDecl {
    pub ident: Ident,
    pub ty: Option<Box<Ty>>,
    pub item: Item,
}

impl Section for ItemDecl {
    #[inline]
    #[must_use]
    fn span(&self) -> Span {
        self.ident.span.to(self.item.span())
    }
}

impl Parser<'_> {
    pub fn parse_item_decl(&mut self) -> ParseResult<ItemDecl> {
        let ident = self.parse_ident()?;
        self.bump_expect(&tk::colon)?;

        let ty = (self.peek().kind != tk::colon)
            .then(|| self.parse_ty())
            .transpose()?
            .map(Box::new);

        self.bump_expect(&tk::colon)?;
        let item = self.parse_item()?;

        Ok(ItemDecl { ident, ty, item })
    }
}
