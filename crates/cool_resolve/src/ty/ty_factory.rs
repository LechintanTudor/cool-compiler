use crate::{
    ArrayTy, FnAbi, FnTy, ManyPtrTy, PtrTy, ResolveContext, SliceTy, TupleTy, TyId, TyKind,
};

impl ResolveContext<'_> {
    pub fn add_ty<K>(&mut self, kind: K) -> TyId
    where
        K: Into<TyKind>,
    {
        self.tys.insert(kind.into())
    }

    #[inline]
    pub fn add_array_ty(&mut self, elem_ty: TyId, len: u64) -> TyId {
        self.add_ty(ArrayTy { elem_ty, len })
    }

    pub fn add_fn_ty<P>(
        &mut self,
        abi: FnAbi,
        param_tys: P,
        is_variadic: bool,
        return_ty: TyId,
    ) -> TyId
    where
        P: IntoIterator<Item = TyId>,
    {
        self.add_ty(FnTy {
            abi,
            param_tys: param_tys.into_iter().collect(),
            is_variadic,
            return_ty,
        })
    }

    #[inline]
    pub fn add_many_ptr_ty(&mut self, pointee_ty: TyId, is_mutable: bool) -> TyId {
        self.add_ty(ManyPtrTy {
            pointee_ty,
            is_mutable,
        })
    }

    #[inline]
    pub fn add_ptr_ty(&mut self, pointee_ty: TyId, is_mutable: bool) -> TyId {
        self.add_ty(PtrTy {
            pointee_ty,
            is_mutable,
        })
    }

    #[inline]
    pub fn add_slice_ty(&mut self, elem_ty: TyId, is_mutable: bool) -> TyId {
        self.add_ty(SliceTy {
            elem_ty,
            is_mutable,
        })
    }

    pub fn add_tuple_ty<E>(&mut self, elem_tys: E) -> TyId
    where
        E: IntoIterator<Item = TyId>,
    {
        self.add_ty(TupleTy {
            elem_tys: elem_tys.into_iter().collect(),
        })
    }
}
