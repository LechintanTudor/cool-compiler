use crate::ItemId;
use derive_more::Display;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Display, Debug)]
#[display(fmt = "{:?}", item_id)]
pub struct StructTy {
    pub item_id: ItemId,
}
