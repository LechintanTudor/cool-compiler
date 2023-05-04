use crate::TyId;
use cool_collections::SmallVecMap;
use cool_lexer::symbols::Symbol;

#[derive(Clone, Default, Debug)]
pub struct StructTy {
    pub fields: SmallVecMap<Symbol, TyId, 7>,
}
