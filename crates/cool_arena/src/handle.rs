use std::fmt;
use std::marker::PhantomData;
use std::num::NonZeroU32;

#[derive(PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Handle<T>
where
    T: ?Sized,
{
    index: NonZeroU32,
    _phantom: PhantomData<*const T>,
}

impl<T> Handle<T>
where
    T: ?Sized,
{
    #[inline]
    pub const fn new(index: u32) -> Option<Self> {
        if index == 0 {
            return None;
        }

        unsafe { Some(Self::new_unchecked(index)) }
    }

    #[inline]
    pub const unsafe fn new_unchecked(index: u32) -> Self {
        Self {
            index: NonZeroU32::new_unchecked(index),
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub const fn index(&self) -> u32 {
        self.index.get()
    }

    #[inline]
    pub const fn as_usize(&self) -> usize {
        self.index.get() as usize
    }

    #[inline]
    pub const fn convert<U>(&self) -> Handle<U>
    where
        U: ?Sized,
    {
        Handle {
            index: self.index,
            _phantom: PhantomData,
        }
    }
}

impl<T> Clone for Handle<T>
where
    T: ?Sized,
{
    fn clone(&self) -> Self {
        Self {
            index: self.index,
            _phantom: PhantomData,
        }
    }
}

impl<T> Copy for Handle<T>
where
    T: ?Sized,
{
    // Empty
}

unsafe impl<T> Send for Handle<T>
where
    T: ?Sized,
{
    // Empty
}

unsafe impl<T> Sync for Handle<T>
where
    T: ?Sized,
{
    // Empty
}

impl<T> fmt::Debug for Handle<T>
where
    T: ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Handle").field(&self.index.get()).finish()
    }
}

#[macro_export]
macro_rules! handle_newtype {
    ($Wrapper:ident wraps $Inner:ty) => {
        #[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
        pub struct $Wrapper(pub(crate) $Inner);

        impl $Wrapper {
            #[inline]
            pub const fn new(index: u32) -> Option<Self> {
                if index == 0 {
                    return None;
                }

                unsafe { Some(Self(<$Inner>::new_unchecked(index))) }
            }

            #[inline]
            pub const unsafe fn new_unchecked(index: u32) -> Self {
                Self(<$Inner>::new_unchecked(index))
            }

            #[inline]
            pub const fn index(&self) -> u32 {
                self.0.index()
            }

            #[inline]
            pub const fn as_usize(&self) -> usize {
                self.0.as_usize()
            }
        }
    };
    ($Wrapper:ident wraps $Inner:ty; Debug) => {
        handle_newtype!($Wrapper wraps $Inner);

        impl std::fmt::Debug for $Wrapper {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.debug_tuple(stringify!($Wrapper)).field(&self.index()).finish()
            }
        }
    };
}
