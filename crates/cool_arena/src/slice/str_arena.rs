use crate::handle::Handle;
use crate::slice::SliceArena;
use std::{fmt, ops};

pub type StrHandle = Handle<str>;

#[derive(Default)]
pub struct StrArena {
    inner: SliceArena<u8>,
}

impl StrArena {
    #[inline]
    pub fn insert(&mut self, str: &str) -> StrHandle {
        self.inner.get_or_insert(str.as_bytes()).convert()
    }

    #[inline]
    pub fn insert_if_not_exists(&mut self, str: &str) -> Option<StrHandle> {
        self.inner
            .insert_if_not_exists(str.as_bytes())
            .map(|handle| handle.convert())
    }

    #[inline]
    pub fn get(&self, handle: StrHandle) -> Option<&str> {
        self.inner
            .get(handle.convert())
            .map(|slice| unsafe { std::str::from_utf8_unchecked(slice) })
    }

    #[inline]
    pub fn get_handle(&self, str: &str) -> Option<StrHandle> {
        self.inner
            .get_handle(str.as_bytes())
            .map(|handle| handle.convert())
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.inner
            .iter()
            .map(|slice| unsafe { std::str::from_utf8_unchecked(slice) })
    }
}

impl ops::Index<StrHandle> for StrArena {
    type Output = str;

    fn index(&self, handle: StrHandle) -> &Self::Output {
        let slice = &self.inner[handle.convert()];
        unsafe { std::str::from_utf8_unchecked(slice) }
    }
}

impl fmt::Debug for StrArena {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}
