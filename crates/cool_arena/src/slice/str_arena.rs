use crate::handle::Handle;
use crate::slice::SliceArena;
use std::fmt;

pub type StrHandle = Handle<str>;

#[derive(Default)]
pub struct StrArena {
    inner: SliceArena<u8>,
}

impl StrArena {
    #[inline]
    pub fn insert(&mut self, str: &str) -> StrHandle {
        self.inner.insert(str.as_bytes()).convert()
    }

    #[inline]
    pub fn insert_if_not_exists(&mut self, str: &str) -> Option<StrHandle> {
        self.inner
            .insert_if_not_exists(str.as_bytes())
            .map(|handle| handle.convert())
    }

    #[inline]
    pub fn get(&self, handle: StrHandle) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.inner.get(handle.convert())) }
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.inner
            .iter()
            .map(|slice| unsafe { std::str::from_utf8_unchecked(slice) })
    }
}

impl fmt::Debug for StrArena {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}
