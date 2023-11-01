use crate::ItemId;
use cool_lexer::Symbol;
use derive_more::{Display, Error};
use smallvec::SmallVec;

pub type ItemResult<T> = Result<T, ItemError>;

#[derive(Clone, Error, Debug, Display)]
pub enum ItemError {
    #[display("Item already exists")]
    AlreadyExists { item_id: ItemId },

    #[display("Item not found")]
    NotFound { path: SmallVec<[Symbol; 4]> },

    #[display("Item is not a module")]
    NotModule { item_id: ItemId },

    #[display("Item is not accessibe from the current module")]
    NotAccessible { item_id: ItemId },

    #[display("Path contains too many super keywords")]
    TooManySuper,
}
