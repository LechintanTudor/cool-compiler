use crate::{Field, ItemId};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct StructTy {
    pub item_id: ItemId,
    pub def: Arc<Mutex<Option<StructTyDef>>>,
}

impl StructTy {
    #[inline]
    #[must_use]
    pub fn has_known_layout(&self) -> bool {
        self.def.lock().unwrap().is_some()
    }
}

impl PartialEq for StructTy {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.item_id == other.item_id
    }
}

impl Eq for StructTy {}

impl Hash for StructTy {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.item_id.hash(state);
    }
}

impl fmt::Display for StructTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "struct")
    }
}

#[derive(Clone, Debug)]
pub struct StructTyDef {
    pub size: u64,
    pub align: u64,
    pub fields: Arc<[Field]>,
}
