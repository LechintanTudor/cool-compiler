use cool_arena::StrHandle;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol(pub(crate) StrHandle);

impl Symbol {
    #[inline]
    pub const fn dummy() -> Self {
        unsafe { Self(StrHandle::new_unchecked(u32::MAX)) }
    }

    #[inline]
    pub(crate) const unsafe fn new_unchecked(index: u32) -> Self {
        Self(StrHandle::new_unchecked(index))
    }

    #[inline]
    pub const fn index(&self) -> u32 {
        self.0.index()
    }
}

impl fmt::Debug for Symbol {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Symbol").field(&self.0.index()).finish()
    }
}
