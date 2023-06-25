use crate::TyId;
use std::collections::BTreeSet;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum VariantTyKind {
    Regular,
    NullablePtr,
}

#[derive(Clone, Eq, Debug)]
pub struct VariantTy {
    pub kind: VariantTyKind,
    pub variants: BTreeSet<TyId>,
}

impl PartialEq for VariantTy {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.variants == other.variants
    }
}

impl Hash for VariantTy {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.variants.hash(state);
    }
}

impl fmt::Display for VariantTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;

        let mut variant_iter = self.variants.iter();

        if let Some(first) = variant_iter.next() {
            write!(f, "{first}")?;
        }

        for other in variant_iter {
            write!(f, " | {other}")?;
        }

        write!(f, ")")
    }
}
