use crate::handle::Handle;
use crate::slice::InternedSlice;
use bumpalo::Bump;
use rustc_hash::FxHashMap;
use std::fmt;
use std::hash::Hash;

pub struct SliceArena<T> {
    bump: Bump,
    handles: FxHashMap<InternedSlice<T>, Handle<[T]>>,
    slices: Vec<InternedSlice<T>>,
}

impl<T> SliceArena<T> {
    pub fn insert(&mut self, slice: &[T]) -> Handle<[T]>
    where
        T: Copy + PartialEq + Eq + Hash,
    {
        if let Some(&handle) = self.handles.get(slice) {
            return handle;
        }

        if self.slices.len() > u32::MAX as usize {
            panic!("ran out of handle indexes");
        }

        let handle = Handle::new(self.slices.len() as u32).unwrap();

        let interned_slice = {
            let slice = self.bump.alloc_slice_copy(slice);
            unsafe { InternedSlice::new(slice) }
        };

        self.handles.insert(interned_slice, handle);
        self.slices.push(interned_slice);

        handle
    }

    #[inline]
    pub fn get(&self, handle: Handle<[T]>) -> &[T] {
        self.slices[handle.index() as usize].as_slice()
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &[T]> {
        self.slices[1..].iter().map(InternedSlice::as_slice)
    }
}

unsafe impl<T> Sync for SliceArena<T>
where
    T: Sync,
{
    // Empty
}

impl<T> Default for SliceArena<T> {
    fn default() -> Self {
        Self {
            bump: Default::default(),
            handles: Default::default(),
            slices: vec![InternedSlice::empty()],
        }
    }
}

impl<T> Drop for SliceArena<T> {
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

impl<T> fmt::Debug for SliceArena<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}
