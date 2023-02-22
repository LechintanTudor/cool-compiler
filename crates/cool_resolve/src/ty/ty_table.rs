use crate::ty::{FnTy, TupleTy, TyKind};
use cool_arena::{Arena, Handle};
use smallvec::SmallVec;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Ty(Handle<TyKind>);

#[derive(Default)]
pub struct TyTable {
    tys: Arena<TyKind>,
}

impl TyTable {
    pub fn mk_tuple<E>(&mut self, elems: E) -> Ty
    where
        E: Iterator<Item = Ty>,
    {
        let ty_kind: TyKind = TupleTy {
            elems: SmallVec::from_iter(elems),
        }
        .into();

        Ty(self.tys.insert(ty_kind))
    }

    pub fn mk_fn<P>(&mut self, params: P, ret: Ty) -> Ty
    where
        P: Iterator<Item = Ty>,
    {
        let ty_kind: TyKind = FnTy {
            args: SmallVec::from_iter(params),
            ret,
        }
        .into();

        Ty(self.tys.insert(ty_kind))
    }

    #[inline]
    pub fn get(&self, ty: Ty) -> &TyKind {
        self.tys.get(ty.0)
    }
}
