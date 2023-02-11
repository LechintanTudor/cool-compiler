use std::num::NonZeroU32;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Symbol(NonZeroU32);

impl Symbol {
    #[inline]
    pub fn new(index: u32) -> Option<Self> {
        NonZeroU32::new(index).map(Self)
    }

    #[inline]
    pub const unsafe fn new_unchecked(index: u32) -> Self {
        Self(NonZeroU32::new_unchecked(index))
    }

    #[inline]
    pub const fn index(&self) -> u32 {
        self.0.get()
    }
}
