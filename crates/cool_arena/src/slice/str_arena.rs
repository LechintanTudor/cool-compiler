use crate::slice::SliceArena;
use crate::Handle;
use std::{fmt, ops};

#[derive(Default)]
pub struct StrArena {
    inner: SliceArena<u8>,
}

impl StrArena {
    #[inline]
    pub fn insert(&mut self, str: &str) -> Handle {
        self.inner.get_or_insert(str.as_bytes())
    }

    #[inline]
    pub fn insert_if_not_exists(&mut self, str: &str) -> Option<Handle> {
        self.inner.insert_if_not_exists(str.as_bytes())
    }

    #[inline]
    pub fn get(&self, handle: Handle) -> Option<&str> {
        self.inner
            .get(handle)
            .map(|slice| unsafe { std::str::from_utf8_unchecked(slice) })
    }

    #[inline]
    pub fn get_handle(&self, str: &str) -> Option<Handle> {
        self.inner.get_handle(str.as_bytes())
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.inner
            .iter()
            .map(|slice| unsafe { std::str::from_utf8_unchecked(slice) })
    }
}

impl ops::Index<Handle> for StrArena {
    type Output = str;

    fn index(&self, handle: Handle) -> &Self::Output {
        let slice = &self.inner[handle];
        unsafe { std::str::from_utf8_unchecked(slice) }
    }
}

impl fmt::Debug for StrArena {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}
