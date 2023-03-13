mod decl;
mod fn_item;
mod item_decl;
mod module_item;
mod use_decl;

pub use self::decl::*;
pub use self::fn_item::*;
pub use self::item_decl::*;
pub use self::module_item::*;
pub use self::use_decl::*;
use crate::ParseTree;
use cool_span::Span;
use paste::paste;

macro_rules! define_item {
    { $($variant:ident,)+ } => {
        paste! {
            #[derive(Clone)]
            pub enum Item {
                $($variant([<$variant Item>]),)+
            }
        }

        impl ParseTree for Item {
            fn span(&self) -> Span {
                match self {
                    $(Self::$variant(i) => i.span(),)+
                }
            }
        }

        impl std::fmt::Debug for Item {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$variant(i) => std::fmt::Debug::fmt(i, f),)+
                }
            }
        }

        paste! {
            $(
                impl From<[<$variant Item>]> for Item {
                    fn from(item: [<$variant Item>]) -> Self {
                        Self::$variant(item)
                    }
                }
            )+
        }
    };
}

define_item! {
    Fn,
    Module,
}
