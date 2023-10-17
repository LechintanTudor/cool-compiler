mod unsafe_bump;

pub use self::unsafe_bump::*;

use ahash::AHashMap;
use std::hash::Hash;
use std::num::NonZeroU32;

pub struct Arena<'a, I, T>
where
    T: ?Sized,
{
    bump: &'a UnsafeBump,
    indexes: AHashMap<&'a T, I>,
    values: Vec<&'a T>,
}

impl<'a, I, T> Arena<'a, I, T>
where
    T: ?Sized,
{
    /// # Safety
    /// The bump must only be used by this arena.
    pub unsafe fn new(bump: &'a UnsafeBump) -> Self {
        Self {
            bump,
            indexes: Default::default(),
            values: Default::default(),
        }
    }

    fn get_next_index(&self) -> I
    where
        I: ArenaIndex,
    {
        NonZeroU32::new((self.values.len() + 1) as u32)
            .map(ArenaIndex::new)
            .unwrap()
    }
}

impl<'a, I, T> Arena<'a, I, T>
where
    I: ArenaIndex,
    T: ?Sized + Eq + Hash,
{
    #[must_use]
    pub fn get(&self, index: I) -> Option<&'a T> {
        let index = index.get().get() as usize - 1;
        self.values.get(index).copied()
    }
}

impl<I, T> Arena<'static, I, T>
where
    T: ?Sized,
{
    pub fn new_leak() -> Self {
        unsafe { Self::new(Box::leak(Box::default())) }
    }
}

impl<'a, I> Arena<'a, I, str>
where
    I: ArenaIndex,
{
    pub fn insert_str(&mut self, value: &str) -> I {
        if let Some(&index) = self.indexes.get(&value) {
            return index;
        }

        let index = self.get_next_index();
        let value = unsafe { self.bump.alloc_str(value) };
        self.indexes.insert(value, index);
        self.values.push(value);

        index
    }
}

/// Strongly-typed index for accessing elements in an [`Arena`].
///
/// # Safety
/// [`ArenaIndex::get`] must return the value passed in [`ArenaIndex::new`] extended to a
/// [`usize`].
pub unsafe trait ArenaIndex: Copy {
    fn new(value: NonZeroU32) -> Self;

    #[must_use]
    fn get(&self) -> NonZeroU32;
}

#[macro_export]
macro_rules! define_arena_index {
    ($Ident:ident) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        pub struct $Ident(::std::num::NonZeroU32);

        unsafe impl $crate::ArenaIndex for $Ident {
            #[inline]
            fn new(value: ::std::num::NonZeroU32) -> Self {
                Self(value)
            }

            #[inline]
            fn get(&self) -> ::std::num::NonZeroU32 {
                self.0
            }
        }
    };
}
