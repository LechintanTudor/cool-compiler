use crate::{TyDef, TyShape};
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct ResolveTy {
    pub shape: TyShape,
    pub def: TyDef,
}

impl PartialEq for ResolveTy {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.shape == other.shape
    }
}

impl Eq for ResolveTy {}

impl Hash for ResolveTy {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.shape.hash(state);
    }
}
