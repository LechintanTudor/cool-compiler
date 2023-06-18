mod alias_item;
mod const_item;
mod enum_item;
mod extern_fn_item;
mod module_item;
mod struct_item;

pub use self::alias_item::*;
pub use self::const_item::*;
pub use self::enum_item::*;
pub use self::extern_fn_item::*;
pub use self::module_item::*;
pub use self::struct_item::*;
use crate::{AbstractFn, ParseResult, Parser};
use cool_lexer::tk;
use cool_span::{Section, Span};
use derive_more::From;
use paste::paste;

macro_rules! define_item {
    { $($Variant:ident,)+ } => {
        paste! {
            #[derive(Clone, From, Debug)]
            pub enum Item {
                $($Variant([<$Variant Item>]),)+
            }
        }

        impl Section for Item {
            fn span(&self) -> Span {
                match self {
                    $(Self::$Variant(i) => i.span(),)+
                }
            }
        }
    };
}

define_item! {
    Alias,
    Const,
    Enum,
    ExternFn,
    Module,
    Struct,
}

impl From<AbstractFn> for Item {
    #[inline]
    fn from(abstract_fn: AbstractFn) -> Self {
        match abstract_fn {
            AbstractFn::ExternFn(f) => f.into(),
            AbstractFn::Fn(f) => ConstItem { expr: f.into() }.into(),
        }
    }
}

impl Parser<'_> {
    pub fn parse_item(&mut self) -> ParseResult<Item> {
        let item: Item = match self.peek().kind {
            tk::KW_EXTERN | tk::KW_FN => self.parse_fn_or_extern_fn_item()?.into(),
            tk::KW_MODULE => self.parse_module_item()?.into(),
            tk::KW_STRUCT => self.parse_struct_item()?.into(),
            tk::KW_TYPE => self.parse_alias_item()?.into(),
            _ => {
                self.peek_error(&[
                    tk::KW_EXTERN,
                    tk::KW_FN,
                    tk::KW_MODULE,
                    tk::KW_STRUCT,
                    tk::KW_TYPE,
                ])?
            }
        };

        Ok(item)
    }
}
