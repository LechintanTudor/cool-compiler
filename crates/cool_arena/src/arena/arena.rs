use crate::arena::InternedRef;
use bumpalo::Bump;
use cool_collections::{Id, IdIndexedVec};
use rustc_hash::FxHashMap;
use std::hash::Hash;
use std::num::NonZeroU32;
use std::{fmt, ops};

pub struct Arena<I, T> {
    bump: Bump,
    ids: FxHashMap<InternedRef<T>, I>,
    refs: IdIndexedVec<I, InternedRef<T>>,
}

impl<I, T> Arena<I, T> {
    pub fn new(dummy: T) -> Self {
        let bump = Bump::new();
        let interned_ref = unsafe { InternedRef::new(bump.alloc(dummy)) };

        Self {
            bump,
            ids: Default::default(),
            refs: IdIndexedVec::new(interned_ref),
        }
    }

    pub fn insert_if_not_exists(&mut self, value: T) -> Option<I>
    where
        I: Id,
        T: Eq + Hash,
    {
        if self.ids.contains_key(&value) {
            return None;
        }

        Some(self.insert_new(value))
    }

    pub fn get_or_insert(&mut self, value: T) -> I
    where
        I: Id,
        T: Eq + Hash,
    {
        if let Some(&id) = self.ids.get(&value) {
            return id;
        }

        self.insert_new(value)
    }

    fn insert_new(&mut self, value: T) -> I
    where
        I: Id,
        T: Eq + Hash,
    {
        debug_assert!(self.ids.get(&value).is_none());

        let interned_ref = unsafe { InternedRef::new(self.bump.alloc(value)) };
        let id = self.refs.push(interned_ref);

        self.ids.insert(interned_ref, id);
        id
    }

    #[inline]
    pub fn get(&self, id: I) -> Option<&T>
    where
        I: Id,
    {
        self.refs.get(id).map(InternedRef::as_ref)
    }

    #[inline]
    pub fn get_id(&self, value: &T) -> Option<I>
    where
        I: Id,
        T: Eq + Hash,
    {
        self.ids.get(value).copied()
    }

    #[inline]
    pub fn contains_id(&self, id: I) -> bool
    where
        I: Id,
    {
        self.refs.contains_id(id)
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.refs.as_slice().iter().map(InternedRef::as_ref)
    }

    pub fn iter_ids(&self) -> impl Iterator<Item = I>
    where
        I: Id,
    {
        (1..((self.refs.len() + 1) as u32)).map(|i| I::from(NonZeroU32::new(i).unwrap()))
    }
}

impl<I, T> Default for Arena<I, T>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<I, T> ops::Index<I> for Arena<I, T>
where
    I: Id,
{
    type Output = T;

    fn index(&self, id: I) -> &Self::Output {
        self.get(id).unwrap()
    }
}

impl<I, T> Drop for Arena<I, T> {
    fn drop(&mut self) {
        if !std::mem::needs_drop::<T>() {
            return;
        }

        for ptr in self.refs.iter().map(InternedRef::as_ptr) {
            unsafe {
                std::ptr::drop_in_place(ptr as *mut T);
            }
        }
    }
}

impl<I, T> fmt::Debug for Arena<I, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.refs.iter()).finish()
    }
}
