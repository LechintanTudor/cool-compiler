use crate::UnsafeBump;
use derive_more::{Deref, Display, From};
use rustc_hash::FxHashSet;
use std::hash::{Hash, Hasher};
use std::{fmt, ptr};

#[derive(Eq, From, Deref, Display, Debug)]
#[deref(forward)]
pub struct InternedValue<'a, T>(&'a T)
where
    T: ?Sized;

impl<T> Clone for InternedValue<'_, T>
where
    T: ?Sized,
{
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T> Copy for InternedValue<'_, T>
where
    T: ?Sized,
{
    // Empty
}

impl<T> Hash for InternedValue<'_, T>
where
    T: ?Sized,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        ptr::hash(self.0, state);
    }
}

impl<T> PartialEq for InternedValue<'_, T>
where
    T: ?Sized,
{
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self.0, other.0)
    }
}

pub struct InternArena<'a, T>
where
    T: ?Sized,
{
    bump: &'a UnsafeBump,
    values: FxHashSet<&'a T>,
}

impl<'a, T> InternArena<'a, T>
where
    T: ?Sized,
{
    pub unsafe fn new(bump: &'a UnsafeBump) -> Self {
        Self {
            bump,
            values: Default::default(),
        }
    }
}

impl<T> InternArena<'static, T>
where
    T: ?Sized,
{
    pub fn new_leak() -> Self {
        Self {
            bump: Box::leak(Box::default()),
            values: Default::default(),
        }
    }
}

impl<'a, T> InternArena<'a, T>
where
    T: ?Sized + Eq + Hash,
{
    #[must_use]
    pub fn contains(&self, value: &T) -> bool {
        self.values.contains(value)
    }

    #[must_use]
    pub fn get(&self, value: &T) -> Option<InternedValue<'a, T>> {
        self.values
            .get(value)
            .map(|&value| InternedValue::from(value))
    }

    pub fn iter(&self) -> impl Iterator<Item = InternedValue<'a, T>> + '_ {
        self.values.iter().copied().map(InternedValue::from)
    }
}

impl<'a, T> InternArena<'a, T>
where
    T: Eq + Hash,
{
    pub fn insert(&mut self, value: T) -> InternedValue<'a, T> {
        if let Some(&value) = self.values.get(&value) {
            return InternedValue::from(value);
        }

        let value = unsafe { self.bump.alloc(value) };
        self.values.insert(value);
        InternedValue::from(value)
    }

    pub fn insert_if_not_exists(&mut self, value: T) -> Option<InternedValue<'a, T>> {
        if self.values.contains(&value) {
            return None;
        }

        let value = unsafe { self.bump.alloc(value) };
        self.values.insert(value);
        Some(InternedValue::from(value))
    }
}

impl<'a, E> InternArena<'a, [E]>
where
    E: Eq + Hash,
{
    pub fn insert_slice(&mut self, value: &[E]) -> InternedValue<'a, [E]>
    where
        E: Copy,
    {
        if let Some(&value) = self.values.get(value) {
            return InternedValue::from(value);
        }

        let value = unsafe { self.bump.alloc_slice_copy(value) };
        self.values.insert(value);
        InternedValue::from(value)
    }

    pub fn insert_slice_if_not_exists(&mut self, value: &[E]) -> Option<InternedValue<'a, [E]>>
    where
        E: Copy,
    {
        if self.values.contains(&value) {
            return None;
        }

        let value = unsafe { self.bump.alloc_slice_copy(value) };
        self.values.insert(value);
        Some(InternedValue::from(value))
    }

    pub fn insert_slice_clone(&mut self, value: &[E]) -> InternedValue<'a, [E]>
    where
        E: Clone,
    {
        if let Some(&value) = self.values.get(value) {
            return InternedValue::from(value);
        }

        let value = unsafe { self.bump.alloc_slice_clone(value) };
        self.values.insert(value);
        InternedValue::from(value)
    }

    pub fn insert_slice_clone_if_not_exists(
        &mut self,
        value: &[E],
    ) -> Option<InternedValue<'a, [E]>>
    where
        E: Copy,
    {
        if self.values.contains(&value) {
            return None;
        }

        let value = unsafe { self.bump.alloc_slice_clone(value) };
        self.values.insert(value);
        Some(InternedValue::from(value))
    }
}

impl<'a> InternArena<'a, str> {
    pub fn insert_str(&mut self, value: &str) -> InternedValue<'a, str> {
        if let Some(&value) = self.values.get(value) {
            return InternedValue::from(value);
        }

        let value = unsafe { self.bump.alloc_str(value) };
        self.values.insert(value);
        InternedValue::from(value)
    }

    pub fn insert_str_if_not_exists(&mut self, value: &str) -> Option<InternedValue<'a, str>> {
        if self.values.contains(value) {
            return None;
        }

        let value = unsafe { self.bump.alloc_str(value) };
        self.values.insert(value);
        Some(InternedValue::from(value))
    }
}

impl<T> fmt::Debug for InternArena<'_, T>
where
    T: ?Sized + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.values, f)
    }
}
