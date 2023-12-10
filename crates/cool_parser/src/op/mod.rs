mod assign_op;
mod binary_op;
mod unary_op;

pub use self::assign_op::*;
pub use self::binary_op::*;
pub use self::unary_op::*;

use derive_more::{Display, Error};

#[derive(Clone, Error, Debug, Display)]
#[display("Invalid operator")]
pub struct InvalidOp;

macro_rules! define_op{
    { $OpName:ident { $($Op:ident => $display:literal from $Punct:ident,)* } } => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        pub enum $OpName {
            $($Op,)*
        }

        impl $OpName {
            #[must_use]
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$Op => $display,)*
                    #[allow(unreachable_patterns)] _ => "",
                }
            }
        }

        impl TryFrom<::cool_lexer::TokenKind> for $OpName {
            type Error = crate::InvalidOp;

            fn try_from(token: ::cool_lexer::TokenKind) -> Result<Self, Self::Error> {
                match token {
                    ::cool_lexer::TokenKind::Punct(punct) => Self::try_from(punct),
                    _ => Err(crate::InvalidOp),
                }
            }
        }

        impl TryFrom<::cool_lexer::Punct> for $OpName {
            type Error = crate::InvalidOp;

            fn try_from(punct: ::cool_lexer::Punct) -> Result<Self, Self::Error> {
                let bin_op = match punct {
                    $(::cool_lexer::Punct::$Punct => Self::$Op,)*
                    _ => return Err(crate::InvalidOp),
                };

                Ok(bin_op)
            }
        }

        impl ::std::fmt::Display for $OpName {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}", self.as_str())
            }
        }
    };
}

use define_op;
