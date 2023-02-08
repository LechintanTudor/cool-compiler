use crate::ty::{InternedTy, Ty};
use bumpalo::Bump;
use rustc_hash::FxHashMap;
use std::borrow::Borrow;

pub type TyIndex = u32;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct TyHandle(TyIndex);

pub struct TyArena {
    bump: Bump,
    handles: FxHashMap<InternedTy, TyHandle>,
    tys: Vec<InternedTy>,
}

impl TyArena {
    pub fn insert(&mut self, ty: Ty) -> TyHandle {
        if let Some(&handle) = self.handles.get(&ty) {
            return handle;
        }

        let ty = unsafe { InternedTy::new(self.bump.alloc(ty)) };
        let handle = TyHandle(self.tys.len() as TyIndex);

        self.handles.insert(ty, handle);
        self.tys.push(ty);

        handle
    }
}

impl Drop for TyArena {
    fn drop(&mut self) {
        for ty in self.tys.iter() {
            unsafe {
                std::ptr::drop_in_place(ty.borrow() as *const Ty as *mut Ty);
            }
        }

        self.tys.clear();
        self.handles.clear();
    }
}

impl TyArena {}
