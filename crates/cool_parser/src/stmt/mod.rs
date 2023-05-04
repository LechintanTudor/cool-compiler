mod assign_stmt;
mod decl_stmt;
mod expr_stmt;

pub use self::assign_stmt::*;
pub use self::decl_stmt::*;
pub use self::expr_stmt::*;
use crate::ParseTree;
use cool_span::Span;
use derive_more::From;
use paste::paste;

macro_rules! define_stmt {
    { $($Variant:ident,)+ } => {
        paste! {
            #[derive(Clone, From, Debug)]
            pub enum Stmt {
                $($Variant([<$Variant Stmt>]),)+
            }
        }

        impl ParseTree for Stmt {
            fn span(&self) -> Span {
                match self {
                    $(Self::$Variant(s) => s.span(),)+
                }
            }
        }
    };
}

define_stmt! {
    Decl,
    Assign,
    Expr,
}
