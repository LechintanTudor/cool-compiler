use crate::arena::InternedRef;
use crate::handle::Handle;
use bumpalo::Bump;
use rustc_hash::FxHashMap;
use std::hash::Hash;
use std::{fmt, ops};

pub struct Arena<T> {
    bump: Bump,
    handles: FxHashMap<InternedRef<T>, Handle>,
    refs: Vec<InternedRef<T>>,
}

impl<T> Arena<T> {
    pub fn new(dummy: T) -> Self {
        let bump = Bump::new();
        let interned_ref = unsafe { InternedRef::new(bump.alloc(dummy)) };

        Self {
            bump,
            handles: Default::default(),
            refs: vec![interned_ref],
        }
    }

    pub fn insert_if_not_exists(&mut self, value: T) -> Option<Handle>
    where
        T: Eq + Hash,
    {
        if self.handles.contains_key(&value) {
            return None;
        }

        Some(self.insert_new(value))
    }

    pub fn get_or_insert(&mut self, value: T) -> Handle
    where
        T: Eq + Hash,
    {
        if let Some(&handle) = self.handles.get(&value) {
            return handle;
        }

        self.insert_new(value)
    }

    fn insert_new(&mut self, value: T) -> Handle
    where
        T: Eq + Hash,
    {
        debug_assert!(self.handles.get(&value).is_none());

        if self.refs.len() > u32::MAX as usize {
            panic!("ran out of handle indexes");
        }

        let handle = Handle::new(self.refs.len() as u32).unwrap();
        let interned_ref = unsafe { InternedRef::new(self.bump.alloc(value)) };

        self.handles.insert(interned_ref, handle);
        self.refs.push(interned_ref);

        handle
    }

    #[inline]
    pub fn get(&self, handle: Handle) -> Option<&T> {
        self.refs
            .get(handle.index() as usize)
            .map(InternedRef::as_ref)
    }

    #[inline]
    pub fn get_handle(&self, value: &T) -> Option<Handle>
    where
        T: Eq + Hash,
    {
        self.handles.get(value).copied()
    }

    #[inline]
    pub fn contains_handle(&self, handle: Handle) -> bool {
        handle.as_usize() < self.refs.len()
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.refs[1..].iter().map(InternedRef::as_ref)
    }

    #[inline]
    pub fn iter_handles(&self) -> impl Iterator<Item = Handle> {
        (1..(self.refs.len() as u32)).map(|index| unsafe { Handle::new_unchecked(index) })
    }

    #[inline]
    pub fn iter_with_handle(&self) -> impl Iterator<Item = (Handle, &T)> {
        self.refs[1..]
            .iter()
            .enumerate()
            .map(|(index, value)| unsafe {
                (Handle::new_unchecked((index + 1) as u32), value.as_ref())
            })
    }
}

impl<T> Default for Arena<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> ops::Index<Handle> for Arena<T> {
    type Output = T;

    fn index(&self, handle: Handle) -> &Self::Output {
        self.get(handle).unwrap()
    }
}

impl<T> Drop for Arena<T> {
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

impl<T> fmt::Debug for Arena<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}
