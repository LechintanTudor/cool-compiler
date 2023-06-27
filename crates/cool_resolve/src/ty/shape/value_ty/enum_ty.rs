use crate::ItemId;
use derive_more::Display;
use std::hash::Hash;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Display, Debug)]
pub struct EnumTy {
    pub item_id: ItemId,
}
