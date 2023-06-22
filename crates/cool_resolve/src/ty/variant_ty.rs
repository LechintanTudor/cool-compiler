use crate::{AnyTy, ResolveTy, TyId, ValueTy};
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

impl VariantTy {
    pub fn to_resolve_ty(&self) -> ResolveTy {
        let (size, align) = self
            .variants
            .iter()
            .map(|variant| (variant.get_size(), variant.get_align()))
            .fold((0, 1), |(old_size, old_align), (size, align)| {
                (old_size.max(size), old_align.max(align))
            });

        let (size, align) = match self.kind {
            VariantTyKind::Regular => (size, align),
            VariantTyKind::NullablePtr => (size, align),
        };

        ResolveTy {
            size,
            align,
            ty: AnyTy::from(ValueTy::from(self.clone())),
        }
    }
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
