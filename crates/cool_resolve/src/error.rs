use crate::{ItemId, TyId};
use cool_lexer::Symbol;
use derive_more::{Display, Error};
use smallvec::SmallVec;

pub type ResolveResult<T> = Result<T, ResolveError>;

#[derive(Clone, Error, Debug, Display)]
pub enum ResolveError {
    #[display("Symbol '{symbol}' already exists")]
    SymbolAlreadyExists { symbol: Symbol },

    #[display("Import path has too many 'super' keywords")]
    ImportIsTooSuper,

    #[display("Item not found")]
    ItemNotFound { path: SmallVec<[Symbol; 4]> },

    #[display("Item is not accessible from the current module")]
    ItemNotAccessible { item_id: ItemId },

    #[display("Item is not a module")]
    ItemNotModule { item_id: ItemId },

    #[display("Item is not a type")]
    ItemNotTy { item_id: ItemId },

    #[display("Item is not a constant")]
    ItemNotConst { item_id: ItemId },

    #[display("Item is not a constant usize")]
    ItemNotUsize { item_id: ItemId },

    #[display("Type is incomplete")]
    TyIsIncomplete { ty_id: TyId },

    #[display("Function has an unknown ABI: '{abi}'")]
    FnAbiIsUnknown { abi: Symbol },
}
