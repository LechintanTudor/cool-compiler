use cool_arena::InternedValue;
use cool_lexer::Symbol;
use derive_more::{Deref, From};
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash, From, Deref)]
#[deref(forward)]
pub struct ItemId(InternedValue<'static, [Symbol]>);

impl ItemId {
    #[inline]
    #[must_use]
    pub fn is_child_of(&self, parent_item: ItemId) -> bool {
        self.starts_with(&parent_item)
    }

    #[inline]
    #[must_use]
    pub fn is_parent_of(&self, child_item: ItemId) -> bool {
        child_item.starts_with(self)
    }
}

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
