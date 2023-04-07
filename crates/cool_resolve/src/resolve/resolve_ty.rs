use crate::resolve::{
    ItemId, ItemKind, ModuleElem, ModuleId, ResolveError, ResolveResult, ResolveTable, ScopeId,
};
use crate::ty::{TyId, TyKind};
use crate::ItemPath;
use cool_lexer::symbols::{sym, Symbol};
use std::ops;

impl ResolveTable {
    #[inline]
    pub fn insert_builtin_ty(&mut self, ty_id: TyId, ty_kind: TyKind) {
        self.tys.insert_builtin(ty_id, ty_kind);
    }

    pub fn insert_builtin_ty_item(
        &mut self,
        symbol: Symbol,
        item_id: ItemId,
        ty_id: TyId,
        ty_kind: TyKind,
    ) {
        self.paths
            .insert_if_not_exists(&[sym::EMPTY, symbol])
            .filter(|&i| i == item_id)
            .unwrap();

        self.modules[ModuleId::for_builtins()].elems.insert(
            symbol,
            ModuleElem {
                is_exported: true,
                item_id,
            },
        );

        self.tys.insert_builtin(ty_id, ty_kind);
        self.items.push_checked(item_id, ItemKind::Ty(ty_id));
    }

    pub fn mk_tuple<E>(&mut self, elems: E) -> TyId
    where
        E: IntoIterator<Item = TyId>,
    {
        self.tys.mk_tuple(elems)
    }

    pub fn mk_fn<P>(&mut self, params: P, ret: TyId) -> TyId
    where
        P: IntoIterator<Item = TyId>,
    {
        self.tys.mk_fn(params, ret)
    }

    pub fn resolve_ty_from_path<'a, P>(&self, scope_id: ScopeId, path: P) -> ResolveResult<TyId>
    where
        P: Into<ItemPath<'a>>,
    {
        let path: ItemPath = path.into();
        let item_id = self.resolve_global(scope_id, path)?;
        let ty_id = self[item_id]
            .as_ty_id()
            .ok_or(ResolveError::not_ty(path.last()))?;
        Ok(ty_id)
    }
}

impl ops::Index<TyId> for ResolveTable {
    type Output = TyKind;

    #[inline]
    fn index(&self, ty_id: TyId) -> &Self::Output {
        &self.tys[ty_id]
    }
}
