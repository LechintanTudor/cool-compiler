use crate::Field;
use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct TupleTy {
    pub fields: Vec<Field>,
}

impl fmt::Display for TupleTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.fields.as_slice() {
            [] => write!(f, "()"),
            [field] => write!(f, "({},)", field.ty_id),
            [first, others @ ..] => {
                write!(f, "({}", first.ty_id)?;

                for other in others {
                    write!(f, ", {}", other.ty_id)?;
                }

                write!(f, ")")
            }
        }
    }
}
