use std::marker::PhantomData;
use std::num::NonZeroU32;
use std::{fmt, ops};

pub trait Id: Copy + From<NonZeroU32> {
    fn inner(&self) -> NonZeroU32;

    #[inline]
    fn as_usize(&self) -> usize {
        self.inner().get() as usize
    }
}

#[macro_export]
macro_rules! id_newtype {
    ($Wrapper:ident) => {
        id_newtype!($Wrapper; nodebug);

        impl std::fmt::Debug for $Wrapper {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple(stringify!($Wrapper))
                    .field(&self.0)
                    .finish()
            }
        }
    };
    ($Wrapper:ident; nodebug) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $Wrapper(pub(crate) std::num::NonZeroU32);

        impl $Wrapper {
            #[inline]
            pub const fn new(index: u32) -> Option<Self> {
                if index == 0 {
                    return None;
                }

                unsafe { Some(Self(std::num::NonZeroU32::new_unchecked(index))) }
            }

            #[inline]
            pub fn new_unwrap(index: u32) -> Self {
                Self(std::num::NonZeroU32::new(index).unwrap())
            }

            #[inline]
            pub const unsafe fn new_unchecked(index: u32) -> Self {
                Self(std::num::NonZeroU32::new_unchecked(index))
            }

            #[inline]
            pub const fn dummy() -> Self {
                unsafe { Self::new_unchecked(u32::MAX) }
            }

            #[inline]
            pub const fn index(&self) -> u32 {
                self.0.get()
            }

            #[inline]
            pub const fn as_usize(&self) -> usize {
                self.0.get() as usize
            }
        }

        impl Default for $Wrapper {
            #[inline]
            fn default() -> Self {
                Self::dummy()
            }
        }

        impl $crate::Id for $Wrapper {
            #[inline]
            fn inner(&self) -> std::num::NonZeroU32 {
                self.0
            }
        }

        impl From<std::num::NonZeroU32> for $Wrapper {
            fn from(inner: std::num::NonZeroU32) -> Self {
                Self(inner)
            }
        }
    };
}

pub struct IdIndexedVec<I, T> {
    inner: Vec<T>,
    _phantom: PhantomData<*const I>,
}

unsafe impl<I, T> Send for IdIndexedVec<I, T>
where
    T: Send,
{
    // Empty
}

unsafe impl<I, T> Sync for IdIndexedVec<I, T>
where
    T: Send,
{
    // Empty
}

impl<I, T> IdIndexedVec<I, T> {
    pub fn new(dummy: T) -> Self {
        Self {
            inner: vec![dummy],
            _phantom: PhantomData,
        }
    }

    pub fn push(&mut self, value: T) -> I
    where
        I: Id,
    {
        let id = I::from(NonZeroU32::new(self.inner.len() as u32).unwrap());
        self.inner.push(value);
        id
    }
    
    pub fn push_checked<I2>(&mut self, expected_id: I2, value: T)
    where
        I: Id,
        I2: Id,
    {
        let id = I::from(NonZeroU32::new(self.inner.len() as u32).unwrap());
        self.inner.push(value);
        assert_eq!(id.as_usize(), expected_id.as_usize());
    }

    pub fn as_slice(&self) -> &[T] {
        &self.inner[1..]
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.inner[1..]
    }

    pub fn contains_id(&self, id: I) -> bool
    where
        I: Id,
    {
        id.as_usize() < self.inner.len()
    }

    pub fn get(&self, id: I) -> Option<&T>
    where
        I: Id,
    {
        self.inner.get(id.as_usize())
    }

    pub fn get_mut(&mut self, id: I) -> Option<&mut T>
    where
        I: Id,
    {
        self.inner.get_mut(id.as_usize())
    }
}

impl<I, T> ops::Deref for IdIndexedVec<I, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<I, T> ops::DerefMut for IdIndexedVec<I, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl<I, T> ops::Index<I> for IdIndexedVec<I, T>
where
    I: Id,
{
    type Output = T;

    fn index(&self, id: I) -> &Self::Output {
        self.get(id).unwrap()
    }
}

impl<I, T> ops::IndexMut<I> for IdIndexedVec<I, T>
where
    I: Id,
{
    fn index_mut(&mut self, id: I) -> &mut Self::Output {
        self.get_mut(id).unwrap()
    }
}

impl<I, T> Default for IdIndexedVec<I, T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            inner: vec![T::default()],
            _phantom: PhantomData,
        }
    }
}

impl<I, T> fmt::Debug for IdIndexedVec<I, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_slice(), f)
    }
}
