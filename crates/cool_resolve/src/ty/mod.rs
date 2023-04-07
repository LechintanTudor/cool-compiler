mod ty_id;
mod ty_kind;

pub use self::ty_id::*;
pub use self::ty_kind::*;
use cool_arena::Arena;
use smallvec::SmallVec;
use std::{fmt, ops};

pub mod tys {
    pub use crate::consts::tys::*;
}

#[derive(Default)]
pub struct TyTable {
    tys: Arena<TyId, TyKind>,
}

impl TyTable {
    pub fn insert_builtin(&mut self, ty_id: TyId, ty_kind: TyKind) {
        let ty_handle = self.tys.insert_if_not_exists(ty_kind).unwrap();
        assert_eq!(ty_handle.index(), ty_id.index());
    }

    pub fn mk_tuple<E>(&mut self, elems: E) -> TyId
    where
        E: IntoIterator<Item = TyId>,
    {
        let elems = SmallVec::from_iter(elems);

        if elems.is_empty() {
            return tys::UNIT;
        }

        let ty_kind: TyKind = TupleTy { elems }.into();

        self.tys.get_or_insert(ty_kind)
    }

    pub fn mk_fn<P>(&mut self, params: P, ret: TyId) -> TyId
    where
        P: IntoIterator<Item = TyId>,
    {
        let ty_kind: TyKind = FnTy {
            params: SmallVec::from_iter(params),
            ret,
        }
        .into();

        self.tys.get_or_insert(ty_kind)
    }
}

impl ops::Index<TyId> for TyTable {
    type Output = TyKind;

    #[inline]
    fn index(&self, ty_id: TyId) -> &Self::Output {
        &self.tys[ty_id]
    }
}

impl fmt::Debug for TyTable {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.tys, f)
    }
}
