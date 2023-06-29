use crate::{TyId, ValueTy};
use smallvec::SmallVec;
use std::collections::BTreeSet;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct VariantTy {
    variants: SmallVec<[TyId; 2]>,
}

impl VariantTy {
    pub fn new<V>(variants: V) -> ValueTy
    where
        V: IntoIterator<Item = TyId>,
    {
        let mut variant_set = BTreeSet::<TyId>::new();

        for variant in variants {
            match variant.as_variant() {
                Some(variant_ty) => {
                    variant_set.extend(variant_ty.variants().iter().copied());
                }
                None => {
                    variant_set.insert(variant);
                }
            }
        }

        match variant_set.len() {
            0 => ValueTy::Unit,
            1 => variant_set.first().unwrap().get_value().clone(),
            _ => {
                ValueTy::from(Self {
                    variants: variant_set.into_iter().collect(),
                })
            }
        }
    }

    #[inline]
    pub fn variants(&self) -> &[TyId] {
        &self.variants
    }
}

impl fmt::Display for VariantTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.variants.as_slice() {
            [] => write!(f, "()"),
            [first, others @ ..] => {
                write!(f, "({}", first)?;

                for other in others {
                    write!(f, " | {}", other)?;
                }

                write!(f, ")")
            }
        }
    }
}
