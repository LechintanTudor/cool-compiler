use crate::CoolIndex;
use std::fmt;
use std::marker::PhantomData;
use std::num::NonZeroU32;
use std::ops::{Deref, DerefMut, Index, IndexMut};

pub struct VecMap<I, T> {
    values: Vec<T>,
    _phantom: PhantomData<I>,
}

impl<I, T> VecMap<I, T> {
    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        &self.values
    }

    #[inline]
    #[must_use]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.values
    }
}

impl<I, T> VecMap<I, T>
where
    I: CoolIndex,
{
    pub fn push(&mut self, value: T) -> I {
        let index = NonZeroU32::new((self.values.len() + 1) as u32)
            .map(I::new)
            .expect("NonZeroU32 overflow");

        self.values.push(value);
        index
    }
}

impl<I, T> Clone for VecMap<I, T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            values: self.values.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<I, T> Default for VecMap<I, T> {
    fn default() -> Self {
        Self {
            values: Default::default(),
            _phantom: PhantomData,
        }
    }
}

impl<I, T> Deref for VecMap<I, T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<I, T> DerefMut for VecMap<I, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl<I, T> Index<I> for VecMap<I, T>
where
    I: CoolIndex,
{
    type Output = T;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        &self.values[index.get_index()]
    }
}

impl<I, T> IndexMut<I> for VecMap<I, T>
where
    I: CoolIndex,
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.values[index.get_index()]
    }
}

impl<I, T> fmt::Debug for VecMap<I, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.as_slice()).finish()
    }
}
