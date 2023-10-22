mod alias_item;
mod struct_item;

pub use self::alias_item::*;
pub use self::struct_item::*;

use crate::{FnExpr, ParseResult, Parser};
use cool_derive::Section;
use cool_lexer::tk;
use derive_more::From;

#[derive(Clone, From, Section, Debug)]
pub enum Item {
    Alias(AliasItem),
    Fn(FnExpr),
    Struct(StructItem),
}

impl Parser<'_> {
    pub fn parse_item(&mut self) -> ParseResult<Item> {
        let item = match self.peek().kind {
            tk::kw_alias => self.parse_alias_item()?.into(),
            tk::kw_extern | tk::kw_fn => self.parse_fn_expr()?.into(),
            tk::kw_struct => self.parse_struct_item()?.into(),
            _ => return self.peek_error(&[tk::kw_alias]),
        };

        Ok(item)
    }
}
