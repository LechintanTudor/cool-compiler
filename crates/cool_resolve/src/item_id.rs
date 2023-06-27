use cool_arena::InternedValue;
use cool_lexer::Symbol;
use derive_more::{Deref, From};
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash, From, Deref)]
#[deref(forward)]
pub struct ItemId(InternedValue<'static, [Symbol]>);

impl fmt::Display for ItemId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self.0 {
            [] => Ok(()),
            [symbol] => write!(f, "{symbol}"),
            [first, others @ ..] => {
                write!(f, "{first}")?;

                for other in others {
                    write!(f, ".{other}")?;
                }

                Ok(())
            }
        }
    }
}

impl fmt::Debug for ItemId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self.0 {
            [] => write!(f, "\"\""),
            [symbol] => write!(f, "\"{symbol}\""),
            [first, others @ ..] => {
                write!(f, "\"{first}")?;

                for other in others {
                    write!(f, ".{other}")?;
                }

                write!(f, "\"")
            }
        }
    }
}
