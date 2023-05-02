mod assign_stmt;
mod decl_stmt;
mod expr_stmt;

pub use self::assign_stmt::*;
pub use self::decl_stmt::*;
pub use self::expr_stmt::*;
use crate::ParseTree;
use cool_span::Span;
use paste::paste;

macro_rules! define_stmt {
    { $($variant:ident,)+ } => {
        paste! {
            #[derive(Clone)]
            pub enum Stmt {
                $($variant([<$variant Stmt>]),)+
            }
        }

        impl ParseTree for Stmt {
            fn span(&self) -> Span {
                match self {
                    $(Self::$variant(s) => s.span(),)+
                }
            }
        }

        impl std::fmt::Debug for Stmt {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$variant(s) => std::fmt::Debug::fmt(s, f),)+
                }
            }
        }

        paste! {
            $(
                impl From<[<$variant Stmt>]> for Stmt {
                    fn from(stmt: [<$variant Stmt>]) -> Self {
                        Self::$variant(stmt)
                    }
                }
            )+
        }
    };
}

define_stmt! {
    Decl,
    Assign,
    Expr,
}
