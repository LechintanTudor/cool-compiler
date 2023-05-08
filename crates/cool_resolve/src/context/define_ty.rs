use crate::{
    tys, FnAbi, FnTy, ItemId, ItemKind, ItemPath, ModuleElem, ModuleId, PointerTy, ResolveContext,
    ResolveError, ResolveResult, ResolveTy, Scope, TupleTy, TyKind,
};
use cool_collections::id_newtype;
use cool_lexer::symbols::{sym, Symbol};
use smallvec::SmallVec;
use std::ops;

id_newtype!(TyId);

impl TyId {
    #[inline]
    pub fn is_inferred(&self) -> bool {
        tys::INFERRED.index() <= self.index() && self.index() <= tys::INFERRED_FLOAT.index()
    }

    #[inline]
    pub fn is_int(&self) -> bool {
        tys::I8.index() <= self.index() && self.index() <= tys::USIZE.index()
    }

    #[inline]
    pub fn is_signed_int(&self) -> bool {
        tys::I8.index() <= self.index() && self.index() <= tys::ISIZE.index()
    }

    #[inline]
    pub fn is_unsigned_int(&self) -> bool {
        tys::U8.index() <= self.index() && self.index() <= tys::USIZE.index()
    }

    #[inline]
    pub fn is_float(&self) -> bool {
        self.index() == tys::F32.index() || self.index() == tys::F64.index()
    }

    #[inline]
    pub fn is_number(&self) -> bool {
        self.is_int() || self.is_float()
    }

    #[inline]
    pub fn resolve_non_inferred(self, expected: Self) -> Option<Self> {
        if expected.is_inferred() {
            if !self.is_inferred() {
                Some(self)
            } else {
                None
            }
        } else {
            let can_resolve = (self == expected)
                || (self == tys::INFERRED)
                || (self == tys::INFERRED_NUMBER && expected.is_number())
                || (self == tys::INFERRED_INT && expected.is_int())
                || (self == tys::INFERRED_FLOAT && expected.is_float());

            can_resolve.then_some(expected)
        }
    }
}

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
    pub fn mk_pointer(&mut self, is_mutable: bool, pointee: TyId) -> TyId {
        self.tys.get_or_insert(TyKind::Pointer(PointerTy {
            is_mutable,
            pointee,
        }))
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
        let ty_id = self[item_id]
            .as_ty_id()
            .ok_or(ResolveError::not_ty(path.last()))?;

        Ok(ty_id)
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
