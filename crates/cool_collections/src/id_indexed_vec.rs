use std::marker::PhantomData;
use std::num::NonZeroU32;
use std::{fmt, ops};

pub trait Id: Copy + Eq + From<NonZeroU32> {
    fn index(&self) -> usize;
}

#[macro_export]
macro_rules! id_newtype {
    ($Wrapper:ident) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        pub struct $Wrapper(std::num::NonZeroU32);

        impl $Wrapper {
            #[inline]
            pub const unsafe fn new_unchecked(value: u32) -> Self {
                Self(std::num::NonZeroU32::new_unchecked(value))
            }

            #[inline]
            pub const fn dummy() -> Self {
                unsafe { Self(std::num::NonZeroU32::new_unchecked(u32::MAX)) }
            }
        }

        impl From<std::num::NonZeroU32> for $Wrapper {
            #[inline]
            fn from(value: std::num::NonZeroU32) -> Self {
                Self(value)
            }
        }

        impl $crate::Id for $Wrapper {
            #[inline]
            fn index(&self) -> usize {
                (self.0.get() - 1) as usize
            }
        }
    };
}

#[derive(Clone)]
pub struct IdIndexedVec<I, T> {
    values: Vec<T>,
    _phantom: PhantomData<*const I>,
}

impl<I, T> IdIndexedVec<I, T> {
    pub const fn new() -> Self {
        Self {
            values: Vec::new(),
            _phantom: PhantomData,
        }
    }

    pub fn push(&mut self, value: T) -> I
    where
        I: Id,
    {
        self.values.push(value);

        NonZeroU32::new(self.values.len() as u32)
            .map(I::from)
            .unwrap()
    }

    pub fn push_checked(&mut self, expected_id: I, value: T)
    where
        I: Id,
    {
        let found_id = self.push(value);

        assert_eq!(
            found_id.index(),
            expected_id.index(),
            "IdIndexedVec::push_checked: unexpected id",
        );
    }
}

impl<I, T> Default for IdIndexedVec<I, T> {
    fn default() -> Self {
        Self {
            values: Default::default(),
            _phantom: PhantomData,
        }
    }
}

impl<I, T> ops::Index<I> for IdIndexedVec<I, T>
where
    I: Id,
{
    type Output = T;

    fn index(&self, id: I) -> &Self::Output {
        &self.values[id.index()]
    }
}

impl<I, T> ops::IndexMut<I> for IdIndexedVec<I, T>
where
    I: Id,
{
    fn index_mut(&mut self, id: I) -> &mut Self::Output {
        &mut self.values[id.index()]
    }
}

impl<I, T> ops::Deref for IdIndexedVec<I, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.values.as_slice()
    }
}

impl<I, T> ops::DerefMut for IdIndexedVec<I, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.values.as_mut_slice()
    }
}

impl<I, T> fmt::Debug for IdIndexedVec<I, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.values, f)
    }
}
