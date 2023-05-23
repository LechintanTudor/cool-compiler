use crate::tys;
use cool_collections::{id_newtype, Id};

id_newtype!(TyId);

impl TyId {
    #[inline]
    pub fn is_inferred(&self) -> bool {
        tys::INFER.index() <= self.index() && self.index() <= tys::INFER_FLOAT.index()
    }

    #[inline]
    pub fn is_divergent(&self) -> bool {
        *self == tys::DIVERGE
    }

    #[inline]
    pub fn is_int(&self) -> bool {
        tys::I8.index() <= self.index() && self.index() <= tys::USIZE.index()
    }

    #[inline]
    pub fn is_signed_int(&self) -> bool {
        tys::I8.index() <= self.index() && self.index() <= tys::ISIZE.index()
    }

    #[inline]
    pub fn is_unsigned_int(&self) -> bool {
        tys::U8.index() <= self.index() && self.index() <= tys::USIZE.index()
    }

    #[inline]
    pub fn is_float(&self) -> bool {
        self.index() == tys::F32.index() || self.index() == tys::F64.index()
    }

    #[inline]
    pub fn is_number(&self) -> bool {
        self.is_int() || self.is_float()
    }
}
