use cool_collections::define_index_newtype;

define_index_newtype!(TyId);

#[derive(Clone, Debug)]
pub enum Ty {
    // Empty
}
