mod ty_config;
mod ty_consts;
mod ty_factory;
mod ty_kind;

pub use self::ty_config::*;
pub use self::ty_consts::*;
pub use self::ty_factory::*;
pub use self::ty_kind::*;

use cool_collections::define_index_newtype;

define_index_newtype!(TyId);

impl TyId {
    #[inline]
    #[must_use]
    pub fn is_any_infer(&self) -> bool {
        [tys::infer, tys::infer_number].contains(self)
    }

    #[inline]
    #[must_use]
    pub fn is_item(&self) -> bool {
        [tys::alias, tys::module].contains(self)
    }

    #[inline]
    #[must_use]
    pub fn is_definable(&self) -> bool {
        !self.is_any_infer() && !self.is_item()
    }

    #[inline]
    #[must_use]
    pub fn is_undefinable(&self) -> bool {
        !self.is_definable()
    }

    #[inline]
    #[must_use]
    pub fn is_int(&self) -> bool {
        tys::i8 <= *self && *self <= tys::usize
    }

    #[inline]
    #[must_use]
    pub fn is_signed_int(&self) -> bool {
        tys::i8 <= *self && *self <= tys::isize
    }

    #[inline]
    #[must_use]
    pub fn is_unsigned_int(&self) -> bool {
        tys::u8 <= *self && *self <= tys::usize
    }

    #[inline]
    #[must_use]
    pub fn is_float(&self) -> bool {
        [tys::f32, tys::f64].contains(self)
    }

    #[inline]
    #[must_use]
    pub fn is_number(&self) -> bool {
        self.is_int() || self.is_float()
    }
}
