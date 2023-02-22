use crate::arena::InternedRef;
use crate::handle::Handle;
use bumpalo::Bump;
use rustc_hash::FxHashMap;
use std::fmt;
use std::hash::Hash;

pub struct Arena<T> {
    bump: Bump,
    handles: FxHashMap<InternedRef<T>, Handle<T>>,
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

    #[inline]
    pub fn get(&self, handle: Handle<T>) -> &T {
        self.refs[handle.index() as usize].as_ref()
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.refs[1..].iter().map(InternedRef::as_ref)
    }
}

impl<T> Arena<T>
where
    T: PartialEq + Eq + Hash,
{
    pub fn insert(&mut self, value: T) -> Handle<T> {
        if let Some(&handle) = self.handles.get(&value) {
            return handle;
        }

        self.insert_new(value)
    }

    fn insert_new(&mut self, value: T) -> Handle<T> {
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
}

impl<T> Default for Arena<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(T::default())
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
