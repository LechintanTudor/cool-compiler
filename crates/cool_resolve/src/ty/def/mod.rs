mod aggregate_ty;
mod primitive_ty_data;

pub use self::aggregate_ty::*;
pub use self::primitive_ty_data::*;

#[derive(Clone, Debug)]
pub enum TyKind {
    Basic,
    Aggregate(AggregateTy),
}

#[derive(Clone, Debug)]
pub struct TyDef {
    pub size: u64,
    pub align: u64,
    pub kind: TyKind,
}

impl TyDef {
    #[inline]
    pub fn is_zero_sized(&self) -> bool {
        self.size == 0
    }
}
