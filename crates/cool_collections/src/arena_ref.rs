use std::borrow::Borrow;
use std::fmt;
use std::hash::{Hash, Hasher};

pub(crate) struct ArenaRef<T>(*const T)
where
    T: ?Sized;

impl<T> ArenaRef<T>
where
    T: ?Sized,
{
    pub unsafe fn new(value: &T) -> Self {
        Self(value as *const _)
    }

    #[must_use]
    pub fn get(&self) -> &T {
        unsafe { &*self.0 }
    }
}

unsafe impl<T> Send for ArenaRef<T>
where
    T: ?Sized + Sync,
{
    // Empty
}

unsafe impl<T> Sync for ArenaRef<T>
where
    T: ?Sized + Sync,
{
    // Empty
}

impl<T> Clone for ArenaRef<T>
where
    T: ?Sized,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for ArenaRef<T>
where
    T: ?Sized,
{
    // Empty
}

impl<T> PartialEq for ArenaRef<T>
where
    T: ?Sized + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.get().eq(other.get())
    }
}

impl<T> Eq for ArenaRef<T>
where
    T: ?Sized + Eq,
{
    // Empty
}

impl<T> Hash for ArenaRef<T>
where
    T: ?Sized + Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.get().hash(state);
    }
}

impl<T> Borrow<T> for ArenaRef<T>
where
    T: ?Sized,
{
    fn borrow(&self) -> &T {
        unsafe { &*self.0 }
    }
}

impl<T> fmt::Debug for ArenaRef<T>
where
    T: ?Sized + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(f)
    }
}

impl<T> fmt::Display for ArenaRef<T>
where
    T: ?Sized + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(f)
    }
}
