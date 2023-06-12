use crate::{
    ArrayTy, Field, FnAbi, FnTy, ItemId, ItemKind, ItemPath, ManyPtrTy, ModuleElem, ModuleId,
    PtrTy, ResolveContext, ResolveError, ResolveErrorKind, ResolveResult, Scope, SliceTy, StructTy,
    TupleTy, TyConsts, TyId, TyMismatch,
};
use cool_lexer::{sym, Symbol};
use smallvec::SmallVec;

impl ResolveContext {
    pub fn insert_primitive_item_ty(&mut self, symbol: Symbol, ty_id: TyId) {
        let item_id = self
            .paths
            .insert_if_not_exists(&[sym::EMPTY, symbol])
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
        self.tys.get_or_insert_value(ArrayTy { len, elem })
    }

    pub fn mk_tuple<F>(&mut self, field_tys: F) -> TyId
    where
        F: IntoIterator<Item = TyId>,
    {
        let fields = field_tys
            .into_iter()
            .enumerate()
            .map(|(i, ty_id)| {
                Field {
                    offset: 0,
                    symbol: Symbol::insert_u32(i as u32),
                    ty_id,
                }
            })
            .collect::<Vec<_>>();

        if fields.is_empty() {
            return self.tys.consts().unit;
        }

        self.tys.get_or_insert_value(TupleTy { fields })
    }

    pub fn mk_struct(&mut self, item_id: ItemId) -> TyId {
        self.tys.get_or_insert_value(StructTy {
            item_id,
            def: Default::default(),
        })
    }

    pub fn mk_fn<P>(&mut self, abi: FnAbi, params: P, is_variadic: bool, ret: TyId) -> TyId
    where
        P: IntoIterator<Item = TyId>,
    {
        self.tys.get_or_insert_value(FnTy {
            abi,
            params: SmallVec::from_iter(params),
            is_variadic,
            ret,
        })
    }

    #[inline]
    pub fn mk_ptr(&mut self, is_mutable: bool, pointee: TyId) -> TyId {
        self.tys.get_or_insert_value(PtrTy {
            pointee,
            is_mutable,
        })
    }

    #[inline]
    pub fn mk_many_ptr(&mut self, is_mutable: bool, pointee: TyId) -> TyId {
        self.tys.get_or_insert_value(ManyPtrTy {
            pointee,
            is_mutable,
        })
    }

    #[inline]
    pub fn mk_slice(&mut self, is_mutable: bool, elem: TyId) -> TyId {
        let ty = SliceTy {
            fields: [
                Field {
                    offset: 0,
                    symbol: sym::PTR,
                    ty_id: self.mk_many_ptr(is_mutable, elem),
                },
                Field {
                    offset: 0,
                    symbol: sym::LEN,
                    ty_id: self.tys.consts().usize,
                },
            ],
        };

        self.tys.get_or_insert_value(ty)
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
