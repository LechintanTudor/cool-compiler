use crate::{ItemId, TyId};
use cool_lexer::symbols::Symbol;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Eq, Debug)]
pub struct Field {
    pub offset: u64,
    pub symbol: Symbol,
    pub ty_id: TyId,
}

impl PartialEq for Field {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol && self.ty_id == other.ty_id
    }
}

impl Hash for Field {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.symbol.hash(state);
        self.ty_id.hash(state);
    }
}

#[derive(Clone, Eq, Debug)]
pub struct StructTy {
    pub item_id: ItemId,
    pub fields: Vec<Field>,
}

impl PartialEq for StructTy {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.item_id == other.item_id
    }
}

impl Hash for StructTy {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.item_id.hash(state);
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct TupleTy {
    pub fields: Vec<Field>,
}
