use crate::ty::Ty;
use std::borrow::Borrow;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy)]
pub struct InternedTy(*const Ty);

impl InternedTy {
    pub unsafe fn new(ty: &Ty) -> Self {
        Self(ty as *const Ty)
    }
}

unsafe impl Send for InternedTy {}
unsafe impl Sync for InternedTy {}

impl PartialEq for InternedTy {
    fn eq(&self, other: &Self) -> bool {
        unsafe { &*self.0 == &*other.0 }
    }
}

impl Eq for InternedTy {}

impl Hash for InternedTy {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        let ty = unsafe { &*self.0 };
        ty.hash(state);
    }
}

impl Borrow<Ty> for InternedTy {
    fn borrow(&self) -> &Ty {
        unsafe { &*self.0 }
    }
}
