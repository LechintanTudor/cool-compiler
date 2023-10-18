mod alias_item;

pub use self::alias_item::*;

use crate::{ParseResult, Parser};
use cool_derive::Section;
use cool_lexer::tk;
use derive_more::From;

#[derive(Clone, From, Section, Debug)]
pub enum Item {
    Alias(AliasItem),
}

impl Parser<'_> {
    pub fn parse_item(&mut self) -> ParseResult<Item> {
        let item = match self.peek().kind {
            tk::kw_alias => self.parse_alias_item()?.into(),
            _ => return self.peek_error(&[tk::kw_alias]),
        };

        Ok(item)
    }
}
