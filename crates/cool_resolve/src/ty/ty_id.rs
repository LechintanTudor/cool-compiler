use crate::ty::tys;
use cool_collections::id_newtype;

id_newtype!(TyId);

impl TyId {
    #[inline]
    pub fn is_inferred(&self) -> bool {
        tys::INFERRED.index() <= self.index() && self.index() <= tys::INFERRED_FLOAT.index()
    }

    #[inline]
    pub fn is_int(&self) -> bool {
        tys::I8.index() <= self.index() && self.index() <= tys::USIZE.index()
    }

    #[inline]
    pub fn is_float(&self) -> bool {
        self.index() == tys::F32.index() || self.index() == tys::F64.index()
    }

    #[inline]
    pub fn resolve_non_inferred(self, expected: Self) -> Option<Self> {
        if expected == tys::INFERRED {
            if !self.is_inferred() {
                Some(self)
            } else {
                None
            }
        } else {
            if self == expected {
                Some(expected)
            } else if self == tys::INFERRED {
                Some(expected)
            } else if self == tys::INFERRED_INT && expected.is_int() {
                Some(expected)
            } else if self == tys::INFERRED_FLOAT && expected.is_float() {
                Some(expected)
            } else {
                None
            }
        }
    }
}
