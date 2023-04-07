use crate::ty::tys;
use cool_collections::id_newtype;

id_newtype!(TyId);

impl TyId {
    #[inline]
    pub fn is_int(&self) -> bool {
        tys::I8.index() <= self.index() && self.index() <= tys::USIZE.index()
    }

    #[inline]
    pub fn is_float(&self) -> bool {
        self.index() == tys::F32.index() || self.index() == tys::F64.index()
    }

    #[inline]
    pub fn is_inferred(&self) -> bool {
        tys::INFERRED.index() <= self.index() && self.index() <= tys::INFERRED_FLOAT.index()
    }

    #[inline]
    pub fn resolve_non_inferred(self, expected: Self) -> Option<Self> {
        if expected == tys::INFERRED && !self.is_inferred() {
            Some(self)
        } else if expected == tys::INFERRED_INT && self.is_int() {
            Some(self)
        } else if expected == tys::INFERRED_FLOAT && self.is_float() {
            Some(self)
        } else if self.is_inferred() {
            Some(expected)
        } else if expected == self {
            Some(expected)
        } else {
            None
        }
    }
}
