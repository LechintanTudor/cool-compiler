mod aggregate_ty;
mod primitive_ty_data;
mod tagged_union_ty;

pub use self::aggregate_ty::*;
pub use self::primitive_ty_data::*;
pub use self::tagged_union_ty::*;
use cool_lexer::Symbol;
use derive_more::From;
use std::sync::Arc;

#[derive(Clone, From, Debug)]
pub enum TyKind {
    Basic,
    Aggregate(AggregateTy),
    TaggedUnion(TaggedUnionTy),
}

impl TyKind {
    #[inline]
    pub fn as_aggregate(&self) -> Option<&AggregateTy> {
        match self {
            Self::Aggregate(aggregate_ty) => Some(aggregate_ty),
            _ => None,
        }
    }

    #[inline]
    pub fn as_tagged_union(&self) -> Option<&TaggedUnionTy> {
        match self {
            Self::TaggedUnion(tagged_union_ty) => Some(tagged_union_ty),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TyDef {
    pub size: u64,
    pub align: u64,
    pub kind: TyKind,
}

impl TyDef {
    pub fn get_aggregate_fields(&self) -> Option<&Arc<[Field]>> {
        self.kind.as_aggregate().map(AggregateTy::fields_arc)
    }

    #[inline]
    pub fn get_aggregate_field(&self, symbol: Symbol) -> Option<&Field> {
        self.kind.as_aggregate()?.get_field(symbol)
    }

    #[inline]
    pub fn is_zero_sized(&self) -> bool {
        self.size == 0
    }
}

pub(crate) fn compute_padding_for_align(offset: u64, align: u64) -> u64 {
    let misalign = offset % align;

    if misalign > 0 {
        align - misalign
    } else {
        0
    }
}
