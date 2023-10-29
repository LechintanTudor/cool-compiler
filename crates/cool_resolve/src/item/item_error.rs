use crate::ItemId;
use derive_more::{Display, Error};

pub type ItemResult<T> = Result<T, ItemError>;

#[derive(Clone, Error, Debug, Display)]
#[display("Item error: {}", self.kind)]
pub struct ItemError {
    pub item_id: ItemId,
    pub kind: ItemErrorKind,
}

#[derive(Clone, Debug, Display)]
pub enum ItemErrorKind {
    #[display("Item already exists")]
    AlreadyExists,

    #[display("Item not found")]
    NotFound,
}
