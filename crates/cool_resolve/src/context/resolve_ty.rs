use crate::{
    ArrayTy, FnAbi, FnTy, ItemId, ItemKind, ItemPath, ManyPtrTy, ModuleElem, ModuleId, PtrTy,
    ResolveContext, ResolveError, ResolveErrorKind, ResolveResult, Scope, SliceTy, StructTy,
    TupleTy, TyConsts, TyId, TyMismatch,
};
use cool_lexer::{sym, Symbol};
use smallvec::SmallVec;

impl ResolveContext {
    pub(crate) fn insert_primitive_item_ty(&mut self, symbol: Symbol, ty_id: TyId) {
        let item_id = self
            .paths
            .insert_slice_if_not_exists(&[sym::EMPTY, symbol])
            .unwrap();

        self.modules[ModuleId::for_builtins()].elems.insert(
            symbol,
            ModuleElem {
                is_exported: true,
                item_id,
            },
        );

        self.items.push(ItemKind::Ty(ty_id));
    }

    pub fn mk_array(&mut self, len: u64, elem: TyId) -> TyId {
        self.tys.insert_value(ArrayTy { len, elem })
    }

    pub fn mk_tuple<E>(&mut self, elems: E) -> TyId
    where
        E: IntoIterator<Item = TyId>,
    {
        self.tys.insert_value(TupleTy {
            elems: elems.into_iter().collect(),
        })
    }

    pub fn mk_struct(&mut self, item_id: ItemId) -> TyId {
        self.tys.insert_value(StructTy { item_id })
    }

    pub fn mk_fn<P>(&mut self, abi: FnAbi, params: P, is_variadic: bool, ret: TyId) -> TyId
    where
        P: IntoIterator<Item = TyId>,
    {
        self.tys.insert_value(FnTy {
            abi,
            params: SmallVec::from_iter(params),
            is_variadic,
            ret,
        })
    }

    pub fn mk_ptr(&mut self, pointee: TyId, is_mutable: bool) -> TyId {
        self.tys.insert_value(PtrTy {
            pointee,
            is_mutable,
        })
    }

    pub fn mk_many_ptr(&mut self, pointee: TyId, is_mutable: bool) -> TyId {
        self.tys.insert_value(ManyPtrTy {
            pointee,
            is_mutable,
        })
    }

    pub fn mk_slice(&mut self, elem: TyId, is_mutable: bool) -> TyId {
        self.tys.insert_value(SliceTy { elem, is_mutable })
    }

    pub fn mk_variant<V>(&mut self, _variants: V) -> TyId
    where
        V: IntoIterator<Item = TyId>,
    {
        todo!()
        // let mut ty_ids = BTreeSet::<TyId>::default();

        // for ty_id in variants {
        //     match ty_id.as_variant() {
        //         Some(variant_ty) => {
        //             ty_ids.extend(variant_ty.variants.iter().cloned());
        //         }
        //         None => {
        //             ty_ids.insert(ty_id);
        //         }
        //     }
        // }

        // match ty_ids.len() {
        //     0 => self.ty_consts().unit,
        //     1 => ty_ids.iter().next().copied().unwrap(),
        //     _ => {
        //         self.tys.insert_value(VariantTy {
        //             kind: VariantTyKind::NullablePtr,
        //             variants: ty_ids.into_iter().collect(),
        //         })
        //     }
        // }
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
        self.tys.resolve_direct_ty_id(found_ty_id, expected_ty_id)
    }

    #[inline]
    pub fn ty_consts(&self) -> &TyConsts {
        self.tys.consts()
    }

    #[inline]
    pub fn iter_value_ty_ids(&self) -> impl Iterator<Item = TyId> + '_ {
        self.tys.iter_value_ty_ids()
    }
}
