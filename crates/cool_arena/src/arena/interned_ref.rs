use std::borrow::Borrow;
use std::convert::AsRef;
use std::fmt;
use std::hash::{Hash, Hasher};

pub struct InternedRef<T> {
    ptr: *const T,
}

impl<T> InternedRef<T> {
    pub const unsafe fn new(object: &T) -> Self {
        Self {
            ptr: object as *const T,
        }
    }

    pub const fn as_ptr(&self) -> *const T {
        self.ptr
    }
}

impl<T> Clone for InternedRef<T> {
    fn clone(&self) -> Self {
        Self { ptr: self.ptr }
    }
}

impl<T> Copy for InternedRef<T> {}

unsafe impl<T> Send for InternedRef<T>
where
    T: Sync,
{
    // Empty
}

unsafe impl<T> Sync for InternedRef<T>
where
    T: Sync,
{
    // Empty
}

impl<T> AsRef<T> for InternedRef<T> {
    fn as_ref(&self) -> &T {
        unsafe { &*self.ptr }
    }
}

impl<T> Borrow<T> for InternedRef<T> {
    fn borrow(&self) -> &T {
        self.as_ref()
    }
}

impl<T> PartialEq for InternedRef<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl<T> Eq for InternedRef<T>
where
    T: Eq,
{
    // Empty
}

impl<T> Hash for InternedRef<T>
where
    T: Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.as_ref().hash(state);
    }
}

impl<T> fmt::Debug for InternedRef<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_ref(), f)
    }
}

impl<T> fmt::Display for InternedRef<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_ref(), f)
    }
}
