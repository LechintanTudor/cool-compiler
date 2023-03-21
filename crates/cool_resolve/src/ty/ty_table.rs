use crate::ty::{tys, FnTy, TupleTy, TyKind};
use crate::ItemId;
use cool_arena::Arena;
use cool_collections::id_newtype;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use std::fmt;

id_newtype!(TyId);

#[derive(Default)]
pub struct TyTable {
    tys: Arena<TyId, TyKind>,
    items: FxHashMap<ItemId, TyId>,
}

impl TyTable {
    pub fn with_builtins() -> Self {
        let mut tys = Self::default();
        tys::insert_builtins(&mut tys);
        tys
    }

    pub fn insert_builtin(&mut self, ty_id: TyId, ty_kind: TyKind) {
        let ty_handle = self.tys.insert_if_not_exists(ty_kind).unwrap();
        assert_eq!(ty_handle.index(), ty_id.index());
    }

    pub fn insert_builtin_item(&mut self, item_id: ItemId, ty_id: TyId, ty_kind: TyKind) {
        let ty_handle = self.tys.insert_if_not_exists(ty_kind).unwrap();
        assert_eq!(ty_handle.index(), ty_id.index());

        assert!(!self.items.contains_key(&item_id));
        self.items.insert(item_id, ty_id);
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
            args: SmallVec::from_iter(params),
            ret,
        }
        .into();

        self.tys.get_or_insert(ty_kind)
    }

    #[inline]
    pub fn get_id_by_item_id(&self, item_id: ItemId) -> Option<TyId> {
        self.items.get(&item_id).map(|&ty_id| ty_id)
    }

    #[inline]
    pub fn get_kind_by_item_id(&self, item_id: ItemId) -> Option<&TyKind> {
        let ty_id = *self.items.get(&item_id)?;
        self.get_kind_by_id(ty_id)
    }

    #[inline]
    pub fn get_kind_by_id(&self, ty_id: TyId) -> Option<&TyKind> {
        self.tys.get(ty_id)
    }

    #[inline]
    pub fn iter_ids(&self) -> impl Iterator<Item = TyId> {
        self.tys.iter_ids()
    }
}

impl fmt::Debug for TyTable {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.items, f)
    }
}
