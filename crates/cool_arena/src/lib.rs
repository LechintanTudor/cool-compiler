mod unsafe_bump;

pub use self::unsafe_bump::*;

use ahash::AHashMap;
use std::fmt;
use std::hash::Hash;
use std::num::NonZeroU32;
use std::ops::Index;

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
    T: Eq + Hash,
{
    pub fn insert(&mut self, value: T) -> I {
        let index = self.get_next_index();
        let value = unsafe { self.bump.alloc(value) };
        self.indexes.insert(value, index);
        self.values.push(value);

        index
    }
}

impl<'a, I, T> Arena<'a, I, T>
where
    I: ArenaIndex,
    T: ?Sized,
{
    #[inline]
    #[must_use]
    pub fn get(&self, index: I) -> Option<&'a T> {
        self.values.get(index.get_index()).copied()
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

impl<'a, I, T> Arena<'a, I, [T]>
where
    I: ArenaIndex,
{
    pub fn insert_slice(&mut self, slice: &[T]) -> I
    where
        T: Copy + Eq + Hash,
    {
        if let Some(&index) = self.indexes.get(slice) {
            return index;
        }

        let index = self.get_next_index();
        let slice = unsafe { self.bump.alloc_slice_copy(slice) };
        self.indexes.insert(slice, index);
        self.values.push(slice);

        index
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

impl<'a, I, T> fmt::Debug for Arena<'a, I, T>
where
    T: fmt::Debug + ?Sized,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&self.values).finish()
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

    #[inline]
    #[must_use]
    fn get_index(&self) -> usize {
        self.get().get() as usize - 1
    }
}

#[macro_export]
macro_rules! define_arena_index {
    ($Ident:ident; $(NoDebug)?) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $Ident(::std::num::NonZeroU32);

        impl $Ident {
            #[inline]
            pub const unsafe fn new_unchecked(index: u32) -> Self {
                Self(::std::num::NonZeroU32::new_unchecked(index))
            }
        }

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
    ($Ident:ident) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        pub struct $Ident(::std::num::NonZeroU32);

        impl $Ident {
            #[inline]
            pub const unsafe fn new_unchecked(index: u32) -> Self {
                Self(::std::num::NonZeroU32::new_unchecked(index))
            }
        }

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

impl<'a, I, T> Index<I> for Arena<'a, I, T>
where
    I: ArenaIndex,
    T: ?Sized,
{
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        self.values.get(index.get_index()).unwrap()
    }
}
