use crate::{IntTy, ItemId, TyId};
use cool_lexer::Symbol;
use derive_more::Display;
use smallvec::SmallVec;
use std::hash::{Hash, Hasher};

#[derive(Clone, Eq, Display, Debug)]
#[display(fmt = "enum")]
pub struct EnumTy {
    pub item_id: ItemId,
    pub storage: TyId,
    pub variants: SmallVec<[Symbol; 4]>,
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
