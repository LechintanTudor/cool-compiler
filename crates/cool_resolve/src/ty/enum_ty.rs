use crate::{IntTy, ItemId};
use cool_lexer::Symbol;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

#[derive(Clone, Eq, Debug)]
pub struct EnumTy {
    pub item_id: ItemId,
    pub storage: IntTy,
    pub variants: Arc<[Symbol]>,
}

impl EnumTy {
    pub const DEFAULT_STORAGE: IntTy = IntTy::U32;
}

impl PartialEq for EnumTy {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.item_id == other.item_id
    }
}

impl Hash for EnumTy {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.item_id.hash(state)
    }
}

impl fmt::Display for EnumTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "enum")?;

        if self.storage != Self::DEFAULT_STORAGE {
            write!(f, "({})", self.storage)?;
        }

        Ok(())
    }
}
