mod alias_item;
mod module_item;
mod struct_item;

pub use self::alias_item::*;
pub use self::module_item::*;
pub use self::struct_item::*;

use crate::{ExternFnExpr, FnExpr, FnOrExternFnExpr, LiteralExpr, ParseResult, Parser};
use cool_derive::Section;
use cool_lexer::{tk, TokenKind};
use derive_more::From;

#[derive(Clone, From, Section, Debug)]
pub enum Item {
    Alias(AliasItem),
    ExternFn(ExternFnExpr),
    Fn(FnExpr),
    Literal(LiteralExpr),
    Module(ModuleItem),
    Struct(StructItem),
}

impl From<FnOrExternFnExpr> for Item {
    #[inline]
    fn from(expr: FnOrExternFnExpr) -> Self {
        match expr {
            FnOrExternFnExpr::Fn(expr) => Self::Fn(expr),
            FnOrExternFnExpr::ExternFn(expr) => Self::ExternFn(expr),
        }
    }
}

impl Parser<'_> {
    pub fn parse_item(&mut self) -> ParseResult<Item> {
        let item = match self.peek().kind {
            tk::kw_alias => self.parse_alias_item()?.into(),
            tk::kw_extern | tk::kw_fn => self.parse_fn_or_extern_fn_expr()?.into(),
            tk::kw_module => self.parse_module_item()?.into(),
            tk::kw_struct => self.parse_struct_item()?.into(),
            TokenKind::Literal(_) => self.parse_literal_expr()?.into(),
            _ => {
                return self.peek_error(&[
                    tk::kw_alias,
                    tk::kw_extern,
                    tk::kw_fn,
                    tk::kw_module,
                    tk::kw_struct,
                ]);
            }
        };

        Ok(item)
    }
}
