use crate::{
    tys, ArrayTy, FnAbi, FnTy, ItemId, ItemKind, ItemPath, ModuleElem, ModuleId, PointerTy,
    ResolveContext, ResolveError, ResolveErrorKind, ResolveResult, ResolveTy, Scope, SliceTy,
    TupleTy, TyId, TyKind, TyMismatch,
};
use cool_lexer::symbols::{sym, Symbol};
use smallvec::SmallVec;
use std::ops;

impl ResolveContext {
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

    #[inline]
    pub fn mk_array(&mut self, len: u64, elem: TyId) -> TyId {
        self.tys.get_or_insert(TyKind::Array(ArrayTy { len, elem }))
    }

    #[inline]
    pub fn mk_pointer(&mut self, is_mutable: bool, pointee: TyId) -> TyId {
        self.tys.get_or_insert(TyKind::Pointer(PointerTy {
            is_mutable,
            pointee,
        }))
    }

    #[inline]
    pub fn mk_slice(&mut self, is_mutable: bool, elem: TyId) -> TyId {
        self.tys
            .get_or_insert(TyKind::Slice(SliceTy { is_mutable, elem }))
    }

    pub fn mk_tuple<E>(&mut self, elems: E) -> TyId
    where
        E: IntoIterator<Item = TyId>,
    {
        let elems = SmallVec::from_iter(elems);

        if elems.is_empty() {
            return tys::UNIT;
        }

        self.tys.get_or_insert(TyKind::Tuple(TupleTy { elems }))
    }

    pub fn mk_fn<P>(&mut self, abi: FnAbi, params: P, is_variadic: bool, ret: TyId) -> TyId
    where
        P: IntoIterator<Item = TyId>,
    {
        self.tys.get_or_insert(TyKind::Fn(FnTy {
            abi,
            params: SmallVec::from_iter(params),
            is_variadic,
            ret,
        }))
    }

    #[inline]
    pub fn mk_struct(&mut self, item_id: ItemId) -> TyId {
        self.tys.get_or_insert(TyKind::StructDecl(item_id))
    }

    pub fn resolve_ty_from_path<'a, P>(&self, scope: Scope, path: P) -> ResolveResult<TyId>
    where
        P: Into<ItemPath<'a>>,
    {
        let path: ItemPath = path.into();
        let item_id = self.resolve_global(scope, path)?;
        let ty_id = self[item_id].as_ty_id().ok_or(ResolveError {
            symbol: path.last(),
            kind: ResolveErrorKind::SymbolNotTy,
        })?;

        Ok(ty_id)
    }

    #[inline]
    pub fn resolve_direct_ty_id(
        &self,
        found_ty_id: TyId,
        expected_ty_id: TyId,
    ) -> Result<TyId, TyMismatch> {
        self.tys
            .resolve_direct_ty_id(found_ty_id, expected_ty_id)
            .ok_or(TyMismatch {
                found_ty_id,
                expected_ty_id,
            })
    }

    #[inline]
    pub fn iter_ty_ids(&self) -> impl Iterator<Item = TyId> {
        self.tys.iter_ty_ids()
    }

    #[inline]
    pub fn is_ty_id_zst(&self, ty_id: TyId) -> bool {
        self.tys.get_resolve_ty(ty_id).is_zst()
    }
}

impl ops::Index<TyId> for ResolveContext {
    type Output = ResolveTy;

    #[inline]
    fn index(&self, ty_id: TyId) -> &Self::Output {
        self.tys.get_resolve_ty(ty_id)
    }
}