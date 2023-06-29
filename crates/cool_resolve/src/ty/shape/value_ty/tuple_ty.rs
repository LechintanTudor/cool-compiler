use crate::{TyId, ValueTy};
use smallvec::SmallVec;
use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct TupleTy {
    elems: SmallVec<[TyId; 2]>,
}

impl TupleTy {
    pub fn new<E>(elems: E) -> ValueTy
    where
        E: IntoIterator<Item = TyId>,
    {
        let elems = SmallVec::from_iter(elems);

        if elems.is_empty() {
            return ValueTy::Unit;
        }

        ValueTy::from(Self { elems })
    }

    #[inline]
    pub fn elems(&self) -> &[TyId] {
        &self.elems
    }
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
