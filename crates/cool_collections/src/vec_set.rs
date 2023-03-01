use smallvec::SmallVec;
use std::{fmt, ops};

#[derive(Clone)]
pub struct SmallVecSet<T, const N: usize> {
    inner: SmallVec<[T; N]>,
}

impl<T, const N: usize> fmt::Debug for SmallVecSet<T, N>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_slice(), f)
    }
}

impl<T, const N: usize> Default for SmallVecSet<T, N> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<T, const N: usize> SmallVecSet<T, N> {
    pub fn insert(&mut self, value: T) -> bool
    where
        T: PartialEq,
    {
        if self.inner.contains(&value) {
            return false;
        }

        self.inner.push(value);
        true
    }

    pub fn clear(&mut self) {
        self.inner.clear()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn as_slice(&self) -> &[T] {
        self.inner.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.inner.as_mut_slice()
    }
}

impl<T, const N: usize> ops::Deref for SmallVecSet<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.inner.as_slice()
    }
}

impl<T, const N: usize> ops::DerefMut for SmallVecSet<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut_slice()
    }
}
