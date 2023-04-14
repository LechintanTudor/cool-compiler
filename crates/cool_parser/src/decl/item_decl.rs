use crate::item::Item;
use crate::{Ident, ParseResult, ParseTree, Parser, Ty};
use cool_lexer::tokens::tk;
use cool_resolve::ItemId;
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct ItemDecl {
    pub ident: Ident,
    pub ty: Option<Ty>,
    pub item_id: ItemId,
    pub item: Item,
}

impl ParseTree for ItemDecl {
    #[inline]
    fn span(&self) -> Span {
        self.ident.span.to(self.item.span())
    }
}

impl Parser<'_> {
    pub fn parse_item_decl(&mut self) -> ParseResult<ItemDecl> {
        let ident = self.parse_ident()?;
        self.bump_expect(&tk::COLON)?;

        let ty = if self.peek().kind != tk::COLON {
            Some(self.parse_ty()?)
        } else {
            None
        };

        self.bump_expect(&tk::COLON)?;
        let item = self.parse_item()?;

        Ok(ItemDecl {
            ident,
            ty,
            item_id: ItemId::dummy(),
            item,
        })
    }
}
