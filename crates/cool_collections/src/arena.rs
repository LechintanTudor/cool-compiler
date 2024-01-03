use crate::arena_ref::ArenaRef;
use crate::CoolIndex;
use bumpalo::Bump;
use std::collections::HashMap;
use std::fmt;
use std::hash::{BuildHasher, Hash};
use std::ops::Index;

pub struct Arena<I, T, S = ahash::RandomState>
where
    T: ?Sized,
{
    bump: Bump,
    indexes: HashMap<ArenaRef<T>, I, S>,
    values: Vec<ArenaRef<T>>,
}

impl<I, T, S> Arena<I, T, S>
where
    T: ?Sized,
{
    pub fn reset(&mut self) {
        self.indexes.clear();
        self.values.clear();
        self.bump.reset();
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.values.iter().map(ArenaRef::get)
    }
}

impl<I, T, S> Arena<I, T, S>
where
    I: CoolIndex,
    T: ?Sized,
{
    pub fn iter_indexes(&self) -> impl Iterator<Item = I> {
        (0..self.len() as u32).map(I::new)
    }
}

impl<I, T, S> Arena<I, T, S>
where
    I: CoolIndex,
    T: ?Sized + Eq + Hash,
    S: BuildHasher,
{
    pub fn insert(&mut self, value: T) -> I
    where
        T: Sized,
    {
        if let Some(index) = self.get_index(&value) {
            return index;
        }

        let index = I::new(self.values.len() as u32);
        let value = unsafe { ArenaRef::new(self.bump.alloc(value)) };

        self.indexes.insert(value, index);
        self.values.push(value);

        index
    }

    #[must_use]
    pub fn contains(&self, value: &T) -> bool {
        self.indexes.contains_key(value)
    }

    #[must_use]
    pub fn get(&self, index: I) -> Option<&T> {
        self.values.get(index.get() as usize).map(ArenaRef::get)
    }

    #[must_use]
    pub fn get_index(&self, value: &T) -> Option<I> {
        self.indexes.get(value).copied()
    }
}

impl<I, T, S> Arena<I, [T], S>
where
    I: CoolIndex,
    T: Copy + Eq + Hash,
    S: BuildHasher,
{
    pub fn insert_slice(&mut self, value: &[T]) -> I {
        if let Some(index) = self.get_index(value) {
            return index;
        }

        let index = I::new(self.values.len() as u32);
        let value = unsafe { ArenaRef::new(self.bump.alloc_slice_copy(value)) };

        self.indexes.insert(value, index);
        self.values.push(value);

        index
    }
}

impl<I, S> Arena<I, str, S>
where
    I: CoolIndex,
    S: BuildHasher,
{
    pub fn insert_str(&mut self, value: &str) -> I {
        if let Some(index) = self.get_index(value) {
            return index;
        }

        let index = I::new(self.values.len() as u32);
        let value = unsafe { ArenaRef::new(self.bump.alloc_str(value)) };

        self.indexes.insert(value, index);
        self.values.push(value);

        index
    }
}

impl<I, T, S> Default for Arena<I, T, S>
where
    T: ?Sized,
    S: Default,
{
    fn default() -> Self {
        Self {
            bump: Bump::default(),
            indexes: HashMap::default(),
            values: Vec::default(),
        }
    }
}

impl<I, T, S> fmt::Debug for Arena<I, T, S>
where
    T: ?Sized + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.values.as_slice()).finish()
    }
}

impl<I, T, S> Index<I> for Arena<I, T, S>
where
    I: CoolIndex,
    T: ?Sized,
{
    type Output = T;

    #[must_use]
    fn index(&self, index: I) -> &Self::Output {
        self.values[index.get() as usize].get()
    }
}
