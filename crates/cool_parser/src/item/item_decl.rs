use crate::item::Item;
use crate::{Ident, ParseResult, ParseTree, Parser, UnexpectedToken};
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

        self.bump_expect(&[tk::COLON])?;
        self.bump_expect(&[tk::COLON])?;

        let item = match self.peek().kind {
            tk::KW_MODULE => Item::Module(self.parse_module_item()?),
            tk::KW_FN => Item::Fn(self.parse_fn_item()?),
            _ => {
                return Err(UnexpectedToken {
                    found: self.peek(),
                    expected: &[tk::KW_MODULE, tk::KW_FN],
                })?
            }
        };

        Ok(ItemDecl { ident, item })
    }
}
