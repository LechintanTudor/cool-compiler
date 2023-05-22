use crate::unsafe_bump::UnsafeBump;
use cool_collections::Id;
use rustc_hash::FxHashMap;
use std::hash::Hash;
use std::num::NonZeroU32;
use std::{fmt, ops};

pub struct Arena<'a, I, T> {
    bump: &'a UnsafeBump,
    indexes: FxHashMap<&'a T, I>,
    values: Vec<&'a T>,
}

impl<'a, I, T> Arena<'a, I, T> {
    /// Safety
    /// The bump must only be used by the arena.
    pub unsafe fn new(bump: &'a UnsafeBump) -> Self {
        Self {
            bump,
            indexes: Default::default(),
            values: Default::default(),
        }
    }
}

impl<I, T> Arena<'static, I, T> {
    pub fn new_leak() -> Self {
        unsafe { Self::new(Box::leak(Box::default())) }
    }
}

impl<'a, I, T> Arena<'a, I, T>
where
    I: Id,
    T: Eq + Hash,
{
    pub fn insert_if_not_exists(&mut self, value: T) -> Option<I>
    where
        I: Id,
        T: Eq + Hash,
    {
        if self.indexes.contains_key(&value) {
            return None;
        }

        Some(self.insert_new(value))
    }

    pub fn insert_checked(&mut self, expected_index: I, value: T)
    where
        I: Id,
        T: Eq + Hash,
    {
        assert!(
            !self.indexes.contains_key(&value),
            "Arena::insert_checked: value already exists"
        );

        let index = self.insert_new(value);

        assert_eq!(
            index.index(),
            expected_index.index(),
            "Arena::insert_checked: unexpected index",
        );
    }

    pub fn get_or_insert(&mut self, value: T) -> I
    where
        I: Id,
        T: Eq + Hash,
    {
        if let Some(&index) = self.indexes.get(&value) {
            return index;
        }

        self.insert_new(value)
    }

    #[must_use]
    pub fn get(&self, index: I) -> Option<&T> {
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

    fn insert_new(&mut self, value: T) -> I {
        debug_assert!(!self.indexes.contains_key(&value));

        let value = unsafe { self.bump.alloc(value) };
        self.values.push(value);

        let index = NonZeroU32::new(self.values.len() as u32)
            .map(I::from)
            .unwrap();

        self.indexes.insert(value, index);
        index
    }
}

impl<I, T> ops::Index<I> for Arena<'_, I, T>
where
    I: Id,
{
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        self.values[index.index()]
    }
}

impl<I, T> fmt::Debug for Arena<'_, I, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.values, f)
    }
}
