use crate::Field;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct TupleTy {
    pub fields: Vec<Field>,
}
