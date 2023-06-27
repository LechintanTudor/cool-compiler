mod aggregate_ty;
mod primitive_ty_data;

pub use self::aggregate_ty::*;
pub use self::primitive_ty_data::*;
use cool_lexer::Symbol;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum TyKind {
    Basic,
    Aggregate(AggregateTy),
}

impl TyKind {
    #[inline]
    pub fn as_aggregate(&self) -> Option<&AggregateTy> {
        match self {
            Self::Aggregate(aggregate_ty) => Some(aggregate_ty),
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
