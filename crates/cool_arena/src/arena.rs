use crate::unsafe_bump::UnsafeBump;
use cool_collections::Id;
use rustc_hash::FxHashMap;
use std::hash::Hash;
use std::num::NonZeroU32;
use std::{fmt, ops};

pub struct Arena<'a, I, T>
where
    T: ?Sized,
{
    bump: &'a UnsafeBump,
    indexes: FxHashMap<&'a T, I>,
    values: Vec<&'a T>,
}

impl<'a, I, T> Arena<'a, I, T>
where
    T: ?Sized,
{
    /// Safety
    /// The bump must only be used by the arena.
    pub unsafe fn new(bump: &'a UnsafeBump) -> Self {
        Self {
            bump,
            indexes: Default::default(),
            values: Default::default(),
        }
    }

    fn get_next_index(&self) -> I
    where
        I: Id,
    {
        NonZeroU32::new((self.values.len() + 1) as u32)
            .map(I::from)
            .unwrap()
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

impl<'a, I, T> Arena<'a, I, T>
where
    I: Id,
    T: ?Sized + Eq + Hash,
{
    #[must_use]
    pub fn get(&self, index: I) -> Option<&'a T> {
        self.values.get(index.index()).copied()
    }

    #[must_use]
    pub fn get_id(&self, value: &T) -> Option<I> {
        self.indexes.get(value).copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = &'a T> + '_ {
        self.values.iter().copied()
    }

    pub fn iter_ids(&self) -> impl Iterator<Item = I> {
        (1..=self.values.len())
            .map(|i| NonZeroU32::new(i as u32).unwrap())
            .map(I::from)
    }

    #[must_use]
    pub fn contains(&self, value: &T) -> bool {
        self.indexes.contains_key(&value)
    }
}

impl<'a, I, T> Arena<'a, I, T>
where
    I: Id,
    T: Eq + Hash,
{
    pub fn insert(&mut self, value: T) -> I {
        if let Some(&index) = self.indexes.get(&value) {
            return index;
        }

        let index = self.get_next_index();
        let value = unsafe { self.bump.alloc(value) };
        self.indexes.insert(value, index);
        self.values.push(value);

        index
    }
}

impl<'a, I, E> Arena<'a, I, [E]>
where
    I: Id,
    E: Eq + Hash,
{
    pub fn insert_slice(&mut self, value: &[E]) -> I
    where
        E: Copy,
    {
        if let Some(&index) = self.indexes.get(&value) {
            return index;
        }

        let index = self.get_next_index();
        let value = unsafe { self.bump.alloc_slice_copy(value) };
        self.indexes.insert(value, index);
        self.values.push(value);

        index
    }

    pub fn insert_slice_if_not_exists(&mut self, value: &[E]) -> Option<I>
    where
        E: Copy,
    {
        if self.indexes.contains_key(value) {
            return None;
        }

        Some(self.insert_slice(value))
    }

    pub fn insert_slice_clone(&mut self, value: &[E]) -> I
    where
        E: Copy,
    {
        if let Some(&index) = self.indexes.get(&value) {
            return index;
        }

        let index = self.get_next_index();
        let value = unsafe { self.bump.alloc_slice_clone(value) };
        self.indexes.insert(value, index);
        self.values.push(value);

        index
    }

    pub fn insert_slice_clone_if_not_exists(&mut self, value: &[E]) -> Option<I>
    where
        E: Copy,
    {
        if self.indexes.contains_key(value) {
            return None;
        }

        Some(self.insert_slice_clone(value))
    }
}

impl<'a, I> Arena<'a, I, str>
where
    I: Id,
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

    pub fn insert_str_if_not_exists(&mut self, value: &str) -> Option<I> {
        if self.indexes.contains_key(value) {
            return None;
        }

        Some(self.insert_str(value))
    }
}

impl<I, T> ops::Index<I> for Arena<'_, I, T>
where
    I: Id,
    T: ?Sized,
{
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        self.values[index.index()]
    }
}

impl<I, T> fmt::Debug for Arena<'_, I, T>
where
    T: ?Sized + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.values, f)
    }
}
