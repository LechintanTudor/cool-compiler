use std::borrow::Borrow;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy)]
pub(crate) struct InternedSlice<T> {
    ptr: *const T,
    len: usize,
}

impl<T> InternedSlice<T> {
    pub const fn empty() -> Self {
        Self {
            ptr: std::mem::align_of::<T>() as *const T,
            len: 0,
        }
    }

    pub unsafe fn new(slice: &[T]) -> Self {
        Self {
            ptr: slice.as_ptr(),
            len: slice.len(),
        }
    }

    pub fn as_ptr(&self) -> *const T {
        self.ptr
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}

unsafe impl<T> Send for InternedSlice<T>
where
    T: Send + Sync,
{
    // Empty
}

unsafe impl<T> Sync for InternedSlice<T>
where
    T: Send + Sync,
{
    // Empty
}

impl<T> PartialEq for InternedSlice<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T> Eq for InternedSlice<T>
where
    T: Eq,
{
    // Empty
}

impl<T> Hash for InternedSlice<T>
where
    T: Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.as_slice().hash(state)
    }
}

impl<T> Borrow<[T]> for InternedSlice<T> {
    fn borrow(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> fmt::Debug for InternedSlice<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_slice(), f)
    }
}
