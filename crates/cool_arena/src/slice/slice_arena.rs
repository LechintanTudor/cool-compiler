use crate::slice::InternedSlice;
use bumpalo::Bump;
use cool_collections::{Id, IdIndexedVec};
use rustc_hash::FxHashMap;
use std::hash::Hash;
use std::{fmt, ops};

pub struct SliceArena<I, T> {
    bump: Bump,
    ids: FxHashMap<InternedSlice<T>, I>,
    slices: IdIndexedVec<I, InternedSlice<T>>,
}

impl<I, T> SliceArena<I, T> {
    #[must_use]
    pub fn insert_if_not_exists(&mut self, slice: &[T]) -> Option<I>
    where
        I: Id,
        T: Copy + Eq + Hash,
    {
        if self.ids.get(slice).is_some() {
            return None;
        }

        Some(self.insert_new(slice))
    }

    pub fn get_or_insert(&mut self, slice: &[T]) -> I
    where
        I: Id,
        T: Copy + Eq + Hash,
    {
        if let Some(&id) = self.ids.get(slice) {
            return id;
        }

        self.insert_new(slice)
    }

    fn insert_new(&mut self, slice: &[T]) -> I
    where
        I: Id,
        T: Copy + Eq + Hash,
    {
        debug_assert!(self.ids.get(slice).is_none());

        let interned_slice = unsafe { InternedSlice::new(self.bump.alloc_slice_copy(slice)) };
        let id = self.slices.push(interned_slice);

        self.ids.insert(interned_slice, id);
        id
    }

    #[inline]
    #[must_use]
    pub fn get(&self, id: I) -> Option<&[T]>
    where
        I: Id,
    {
        self.slices.get(id).map(InternedSlice::as_slice)
    }

    #[inline]
    #[must_use]
    pub fn get_id(&self, slice: &[T]) -> Option<I>
    where
        I: Id,
        T: Copy + Eq + Hash,
    {
        self.ids.get(slice).copied()
    }

    #[inline]
    #[must_use]
    pub fn contains(&self, slice: &[T]) -> bool
    where
        T: Eq + Hash,
    {
        unsafe { self.ids.contains_key(&InternedSlice::new(slice)) }
    }

    #[inline]
    #[must_use]
    pub fn contains_id(&self, id: I) -> bool
    where
        I: Id,
    {
        self.slices.contains_id(id)
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &[T]> {
        self.slices.as_slice().iter().map(InternedSlice::as_slice)
    }
}

unsafe impl<I, T> Sync for SliceArena<I, T>
where
    I: Sync,
    T: Sync,
{
    // Empty
}

impl<I, T> Default for SliceArena<I, T> {
    fn default() -> Self {
        Self {
            bump: Default::default(),
            ids: Default::default(),
            slices: IdIndexedVec::new(InternedSlice::empty()),
        }
    }
}

impl<I, T> ops::Index<I> for SliceArena<I, T>
where
    I: Id,
{
    type Output = [T];

    fn index(&self, id: I) -> &Self::Output {
        self.get(id).unwrap()
    }
}

impl<I, T> Drop for SliceArena<I, T> {
    fn drop(&mut self) {
        if !std::mem::needs_drop::<T>() {
            return;
        }

        for slice in self.slices.iter() {
            for i in 0..slice.len() {
                unsafe {
                    std::ptr::drop_in_place(slice.as_ptr().add(i) as *mut T);
                }
            }
        }
    }
}

impl<I, T> fmt::Debug for SliceArena<I, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.slices.iter()).finish()
    }
}
