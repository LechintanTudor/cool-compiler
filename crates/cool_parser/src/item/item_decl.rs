use crate::item::Item;
use crate::{Ident, ParseResult, ParseTree, Parser};
use cool_lexer::tokens::tk;
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct ItemDecl {
    pub ident: Ident,
    pub item: Item,
}

impl ParseTree for ItemDecl {
    fn span(&self) -> Span {
        self.ident.span.to(self.item.span())
    }
}

impl Parser<'_> {
    pub fn parse_item_decl(&mut self) -> ParseResult<ItemDecl> {
        let ident = self.parse_ident()?;

        self.bump_expect(&tk::COLON)?;
        self.bump_expect(&tk::COLON)?;

        let item: Item = match self.peek().kind {
            tk::KW_MODULE => self.parse_module_item()?.into(),
            tk::KW_FN | tk::KW_EXTERN => self.parse_fn_or_extern_fn_item()?,
            _ => self.peek_error(&[tk::KW_MODULE, tk::KW_FN, tk::KW_EXTERN])?,
        };

        Ok(ItemDecl { ident, item })
    }
}
