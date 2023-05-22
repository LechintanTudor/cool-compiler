use crate::unsafe_bump::UnsafeBump;
use cool_collections::Id;
use rustc_hash::FxHashMap;
use std::hash::Hash;
use std::num::NonZeroU32;
use std::{fmt, ops};

pub struct SliceArena<'a, I, T> {
    bump: &'a UnsafeBump,
    ids: FxHashMap<&'a [T], I>,
    values: Vec<&'a [T]>,
}

impl<'a, I, T> SliceArena<'a, I, T> {
    /// Safety
    /// The bump must only be used by the arena.
    pub unsafe fn new(bump: &'a UnsafeBump) -> Self {
        Self {
            bump,
            ids: Default::default(),
            values: Default::default(),
        }
    }
}

impl<I, T> SliceArena<'static, I, T> {
    pub fn new_leak() -> Self {
        unsafe { Self::new(Box::leak(Box::default())) }
    }
}

impl<'a, I, T> SliceArena<'a, I, T>
where
    I: Id,
    T: Eq + Hash,
{
    pub fn insert_if_not_exists(&mut self, values: &[T]) -> Option<I>
    where
        I: Id,
        T: Copy + Eq + Hash,
    {
        if self.ids.contains_key(values) {
            return None;
        }

        Some(self.insert_new(values))
    }

    pub fn insert_checked(&mut self, expected_id: I, values: &[T])
    where
        I: Id,
        T: Copy + Eq + Hash,
    {
        assert!(
            !self.ids.contains_key(values),
            "SliceArena::insert_checked: value already exists"
        );

        let id = self.insert_new(values);

        assert_eq!(
            id.index(),
            expected_id.index(),
            "SliceArena::insert_checked: unexpected id",
        );
    }

    pub fn get_or_insert(&mut self, values: &[T]) -> I
    where
        I: Id,
        T: Copy + Eq + Hash,
    {
        if let Some(&id) = self.ids.get(&values) {
            return id;
        }

        self.insert_new(values)
    }

    #[must_use]
    pub fn get(&self, id: I) -> Option<&'a [T]> {
        self.values.get(id.index()).copied()
    }

    #[must_use]
    pub fn get_id(&self, values: &[T]) -> Option<I> {
        self.ids.get(values).copied()
    }

    #[must_use]
    pub fn contains(&self, values: &[T]) -> bool {
        self.ids.contains_key(values)
    }

    pub fn iter(&self) -> impl Iterator<Item = &'a [T]> + '_ {
        self.values.iter().copied()
    }

    pub fn iter_ids(&self) -> impl Iterator<Item = I> {
        (1..=self.values.len())
            .map(|i| NonZeroU32::new(i as u32).unwrap())
            .map(I::from)
    }

    fn insert_new(&mut self, values: &[T]) -> I
    where
        T: Copy,
    {
        debug_assert!(!self.ids.contains_key(&values));

        let values = unsafe { self.bump.alloc_slice(values) };
        self.values.push(values);

        let id = NonZeroU32::new(self.values.len() as u32)
            .map(I::from)
            .unwrap();

        self.ids.insert(values, id);
        id
    }
}

impl<I, T> ops::Index<I> for SliceArena<'_, I, T>
where
    I: Id,
{
    type Output = [T];

    fn index(&self, id: I) -> &Self::Output {
        self.values[id.index()]
    }
}

impl<I, T> fmt::Debug for SliceArena<'_, I, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.values, f)
    }
}
