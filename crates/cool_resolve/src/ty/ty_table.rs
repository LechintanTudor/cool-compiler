use crate::item::ItemId;
use crate::ty::{tys, FnTy, TupleTy, TyKind};
use cool_arena::{handle_newtype, Arena};
use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use std::fmt;

handle_newtype!(TyId; Debug);

#[derive(Default)]
pub struct TyTable {
    tys: Arena<TyKind>,
    items: FxHashMap<ItemId, TyId>,
}

impl TyTable {
    pub fn with_builtins() -> Self {
        let mut tys = Self::default();
        tys::insert_builtins(&mut tys);
        tys
    }

    pub fn insert_builtin(&mut self, item_id: ItemId, ty_id: TyId, ty_kind: TyKind) {
        let ty_handle = self.tys.insert_if_not_exists(ty_kind).unwrap();
        assert_eq!(ty_handle.index(), ty_id.index());

        assert!(!self.items.contains_key(&item_id));
        self.items.insert(item_id, ty_id);
    }

    pub fn mk_tuple<E>(&mut self, elems: E) -> TyId
    where
        E: IntoIterator<Item = TyId>,
    {
        let ty_kind: TyKind = TupleTy {
            elems: SmallVec::from_iter(elems),
        }
        .into();

        TyId(self.tys.get_or_insert(ty_kind))
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

        TyId(self.tys.get_or_insert(ty_kind))
    }

    #[inline]
    pub fn get_ty_id_by_item_id(&self, item_id: ItemId) -> Option<TyId> {
        self.items.get(&item_id).map(|&ty_id| ty_id)
    }

    #[inline]
    pub fn get_by_item_id(&self, item_id: ItemId) -> Option<&TyKind> {
        let ty_id = *self.items.get(&item_id)?;
        self.get_by_id(ty_id)
    }

    #[inline]
    pub fn get_by_id(&self, ty_id: TyId) -> Option<&TyKind> {
        self.tys.get(ty_id.0)
    }
}

impl fmt::Debug for TyTable {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.items, f)
    }
}
