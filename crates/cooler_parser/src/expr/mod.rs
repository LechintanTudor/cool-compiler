use cool_collections::define_index_newtype;

define_index_newtype!(ExprId);

#[derive(Clone, Debug)]
pub enum Expr {
    // Empty
}
