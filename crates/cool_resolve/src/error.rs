use crate::{ItemId, TyId};
use cool_collections::SmallVec;
use cool_lexer::Symbol;
use derive_more::{Display, Error};

pub type ResolveResult<T = ()> = Result<T, ResolveError>;

#[derive(Clone, Error, Debug, Display)]
pub enum ResolveError {
    #[display("Symbol '{symbol}' already exists")]
    SymbolAlreadyExists { symbol: Symbol },

    #[display("Import path has too many 'super' keywords")]
    ImportIsTooSuper,

    #[display("Item not found")]
    ItemNotFound { path: SmallVec<Symbol, 4> },

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

    #[display("Type is not a function")]
    TyNotFn { ty_id: TyId },

    #[display("Types cannot be unified")]
    TysCannotBeUnified { ty_id: TyId, expected_ty_id: TyId },

    #[display("Struct has duplicared field '{field}'")]
    StructHasDuplicatedField { field: Symbol },

    #[display("Function has an unknown ABI: '{abi}'")]
    FnAbiIsUnknown { abi: Symbol },

    #[display("Function abi mismatch")]
    FnAbiMismatch,

    #[display("Function parameter count mismatch: {found}, {expected}")]
    FnParamCountMismatch { found: u32, expected: u32 },

    #[display("Function parameter type mismatch")]
    FnParamTyMimatch { found: TyId, expected: TyId },

    #[display("Function parameter type is missing")]
    FnParamTyMissing,

    #[display("Function variadic mismatch")]
    FnVariadicMismatch { found: bool },

    #[display("Function return type mismatch")]
    FnReturnTyMismatch { found: TyId, expected: TyId },
}
