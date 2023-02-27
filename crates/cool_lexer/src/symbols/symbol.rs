use cool_arena::StrHandle;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol(pub(crate) StrHandle);

impl Symbol {
    #[inline]
    pub(crate) const unsafe fn new_unchecked(index: u32) -> Self {
        Self(StrHandle::new_unchecked(index))
    }

    #[inline]
    pub const fn index(&self) -> u32 {
        self.0.index()
    }

    #[inline]
    pub const fn as_usize(&self) -> usize {
        self.0.as_usize()
    }
}
