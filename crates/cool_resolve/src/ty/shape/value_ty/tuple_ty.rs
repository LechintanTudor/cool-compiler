use crate::TyId;
use smallvec::SmallVec;
use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct TupleTy {
    pub elems: SmallVec<[TyId; 2]>,
}

impl fmt::Display for TupleTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.elems.as_slice() {
            [] => write!(f, "()"),
            [first] => write!(f, "({},)", first),
            [first, others @ ..] => {
                write!(f, "({}", first)?;

                for other in others {
                    write!(f, ", {}", other)?;
                }

                write!(f, ")")
            }
        }
    }
}
