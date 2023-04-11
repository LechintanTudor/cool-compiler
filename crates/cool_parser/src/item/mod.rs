mod const_item;
mod extern_fn_item;
mod module_item;
mod struct_item;

pub use self::const_item::*;
pub use self::extern_fn_item::*;
pub use self::module_item::*;
pub use self::struct_item::*;
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
    Module,
    Struct,
    ExternFn,
    Const,
}
