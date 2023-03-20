use crate::slice::SliceArena;
use cool_collections::Id;
use std::{fmt, ops};

#[derive(Default)]
pub struct StrArena<I> {
    inner: SliceArena<I, u8>,
}

impl<I> StrArena<I> {
    #[inline]
    pub fn get_or_insert(&mut self, str: &str) -> I
    where
        I: Id,
    {
        self.inner.get_or_insert(str.as_bytes())
    }

    #[inline]
    pub fn insert_if_not_exists(&mut self, str: &str) -> Option<I>
    where
        I: Id,
    {
        self.inner.insert_if_not_exists(str.as_bytes())
    }

    #[inline]
    pub fn get(&self, id: I) -> Option<&str>
    where
        I: Id,
    {
        self.inner
            .get(id)
            .map(|slice| unsafe { std::str::from_utf8_unchecked(slice) })
    }

    #[inline]
    pub fn get_id(&self, str: &str) -> Option<I>
    where
        I: Id,
    {
        self.inner.get_id(str.as_bytes())
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.inner
            .iter()
            .map(|slice| unsafe { std::str::from_utf8_unchecked(slice) })
    }
}

impl<I> ops::Index<I> for StrArena<I>
where
    I: Id,
{
    type Output = str;

    fn index(&self, id: I) -> &Self::Output {
        let slice = &self.inner[id];
        unsafe { std::str::from_utf8_unchecked(slice) }
    }
}

impl<I> fmt::Debug for StrArena<I> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}
