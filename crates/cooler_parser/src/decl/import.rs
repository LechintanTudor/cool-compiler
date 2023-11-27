use crate::Ident;
use cool_collections::{define_index_newtype, SmallVec};

define_index_newtype!(ImportId);

#[derive(Clone, Debug)]
pub struct Import {
    pub path: SmallVec<Ident, 2>,
    pub alias: Option<Ident>,
}
