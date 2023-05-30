use crate::{
    tys, AnyTy, ArrayTy, FnAbi, FnTy, ItemId, ItemKind, ItemPath, ManyPtrTy, ModuleElem, ModuleId,
    PtrTy, ResolveContext, ResolveError, ResolveErrorKind, ResolveResult, ResolveTy, Scope,
    SliceTy, TupleTy, TyId, TyMismatch, ValueTy,
};
use cool_lexer::symbols::{sym, Symbol};
use smallvec::SmallVec;
use std::ops;

impl ResolveContext {
    pub fn insert_builtin_ty(&mut self, ty_id: TyId, ty_kind: AnyTy) {
        self.tys.insert_builtin(ty_id, ty_kind);
    }

    pub fn insert_builtin_ty_item(
        &mut self,
        symbol: Symbol,
        item_id: ItemId,
        ty_id: TyId,
        ty_kind: AnyTy,
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
        let ty = ValueTy::Array(ArrayTy { len, elem });
        self.tys.get_or_insert(ty.into())
    }

    #[inline]
    pub fn mk_ptr(&mut self, is_mutable: bool, pointee: TyId) -> TyId {
        let ty = ValueTy::Ptr(PtrTy {
            is_mutable,
            pointee,
        });
        self.tys.get_or_insert(ty.into())
    }

    #[inline]
    pub fn mk_many_ptr(&mut self, is_mutable: bool, pointee: TyId) -> TyId {
        let ty = ValueTy::ManyPtr(ManyPtrTy {
            is_mutable,
            pointee,
        });

        self.tys.get_or_insert(ty.into())
    }

    #[inline]
    pub fn mk_slice(&mut self, is_mutable: bool, elem: TyId) -> TyId {
        let ty = ValueTy::Slice(SliceTy { is_mutable, elem });
        self.tys.get_or_insert(ty.into())
    }

    pub fn mk_tuple<E>(&mut self, elems: E) -> TyId
    where
        E: IntoIterator<Item = TyId>,
    {
        let elems = SmallVec::from_iter(elems);

        if elems.is_empty() {
            return tys::UNIT;
        }

        let ty = ValueTy::Tuple(TupleTy { elems });
        self.tys.get_or_insert(ty.into())
    }

    pub fn mk_fn<P>(&mut self, abi: FnAbi, params: P, is_variadic: bool, ret: TyId) -> TyId
    where
        P: IntoIterator<Item = TyId>,
    {
        let ty = ValueTy::Fn(FnTy {
            abi,
            params: SmallVec::from_iter(params),
            is_variadic,
            ret,
        });

        self.tys.get_or_insert(ty.into())
    }

    #[inline]
    pub fn mk_struct(&mut self, item_id: ItemId) -> TyId {
        self.tys.get_or_insert(AnyTy::StructDecl(item_id))
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
    pub fn iter_resolve_ty_ids(&self) -> impl Iterator<Item = TyId> + '_ {
        self.tys.iter_resolve_ty_ids()
    }

    #[inline]
    pub fn ty_is_zero_sized(&self, ty_id: TyId) -> bool {
        self.tys.get_resolve_ty(ty_id).unwrap().is_zero_sized()
    }
}

impl ops::Index<TyId> for ResolveContext {
    type Output = ResolveTy;

    #[inline]
    fn index(&self, ty_id: TyId) -> &Self::Output {
        self.tys.get_resolve_ty(ty_id).unwrap()
    }
}
