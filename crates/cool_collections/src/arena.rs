use crate::{CoolIndex, UnsafeBump};
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

    pub fn iter_indexes(&self) -> impl Iterator<Item = I>
    where
        I: CoolIndex,
    {
        (1..(self.values.len() as u32 + 1)).map(|i| CoolIndex::new(NonZeroU32::new(i).unwrap()))
    }

    fn get_next_index(&self) -> I
    where
        I: CoolIndex,
    {
        NonZeroU32::new((self.values.len() + 1) as u32)
            .map(CoolIndex::new)
            .unwrap()
    }
}

impl<'a, I, T> Arena<'a, I, T>
where
    I: CoolIndex,
    T: Eq + Hash,
{
    pub fn insert(&mut self, value: T) -> I {
        if let Some(index) = self.get_index(&value) {
            return index;
        }

        let index = self.get_next_index();
        let value = unsafe { self.bump.alloc(value) };
        self.indexes.insert(value, index);
        self.values.push(value);

        index
    }
}

impl<'a, I, T> Arena<'a, I, T>
where
    I: CoolIndex,
    T: ?Sized,
{
    #[must_use]
    pub fn get(&self, index: I) -> Option<&'a T> {
        self.values.get(index.get_index()).copied()
    }
}

impl<'a, I, T> Arena<'a, I, T>
where
    I: CoolIndex,
    T: ?Sized + Eq + Hash,
{
    #[must_use]
    pub fn get_index(&self, value: &T) -> Option<I> {
        self.indexes.get(value).copied()
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
    I: CoolIndex,
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
    I: CoolIndex,
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

impl<'a, I, T> Index<I> for Arena<'a, I, T>
where
    I: CoolIndex,
    T: ?Sized,
{
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        self.values.get(index.get_index()).unwrap()
    }
}
