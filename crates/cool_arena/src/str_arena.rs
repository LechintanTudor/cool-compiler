use crate::unsafe_bump::UnsafeBump;
use cool_collections::Id;
use rustc_hash::FxHashMap;
use std::num::NonZeroU32;
use std::{fmt, ops};

pub struct StrArena<'a, I> {
    bump: &'a UnsafeBump,
    ids: FxHashMap<&'a str, I>,
    values: Vec<&'a str>,
}

impl<'a, I> StrArena<'a, I> {
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

impl<I> StrArena<'static, I> {
    pub fn new_leak() -> Self {
        unsafe { Self::new(Box::leak(Box::default())) }
    }
}

impl<'a, I> StrArena<'a, I>
where
    I: Id,
{
    pub fn insert_if_not_exists(&mut self, value: &str) -> Option<I>
    where
        I: Id,
    {
        if self.ids.contains_key(value) {
            return None;
        }

        Some(self.insert_new(value))
    }

    pub fn insert_checked(&mut self, expected_id: I, value: &str)
    where
        I: Id,
    {
        assert!(
            !self.ids.contains_key(value),
            "StrArena::insert_checked: value already exists"
        );

        let id = self.insert_new(value);

        assert_eq!(
            id.index(),
            expected_id.index(),
            "StrArena::insert_checked: unexpected id",
        );
    }

    pub fn get_or_insert(&mut self, value: &str) -> I
    where
        I: Id,
    {
        if let Some(&id) = self.ids.get(&value) {
            return id;
        }

        self.insert_new(value)
    }

    #[inline]
    #[must_use]
    pub fn get(&self, id: I) -> Option<&'a str> {
        self.values.get(id.index()).copied()
    }

    #[inline]
    #[must_use]
    pub fn get_id(&self, value: &str) -> Option<I> {
        self.ids.get(value).copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = &'a str> + '_ {
        self.values.iter().copied()
    }

    pub fn iter_ids(&self) -> impl Iterator<Item = I> {
        (1..=self.values.len())
            .map(|i| NonZeroU32::new(i as u32).unwrap())
            .map(I::from)
    }

    fn insert_new(&mut self, value: &str) -> I {
        debug_assert!(!self.ids.contains_key(&value));

        let value = unsafe { self.bump.alloc_str(value) };
        self.values.push(value);

        let id = NonZeroU32::new(self.values.len() as u32)
            .map(I::from)
            .unwrap();

        self.ids.insert(value, id);
        id
    }
}

impl<I> ops::Index<I> for StrArena<'_, I>
where
    I: Id,
{
    type Output = str;

    #[inline]
    fn index(&self, id: I) -> &Self::Output {
        self.values[id.index()]
    }
}

impl<I> fmt::Debug for StrArena<'_, I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.values, f)
    }
}
