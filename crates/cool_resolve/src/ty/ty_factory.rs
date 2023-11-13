use crate::{
    tys, ArrayTy, FnAbi, FnTy, ManyPtrTy, PtrTy, ResolveContext, SliceTy, TupleTy, TyId, TyKind,
    VariantTy,
};
use cool_collections::SmallVec;

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
        let elem_tys = elem_tys.into_iter().collect::<SmallVec<_, 4>>();

        if elem_tys.is_empty() {
            return tys::unit;
        }

        self.add_ty(TupleTy { elem_tys })
    }

    pub fn add_variant_ty<V>(&mut self, variant_tys: V) -> TyId
    where
        V: IntoIterator<Item = TyId>,
    {
        let mut non_variant_tys = SmallVec::<TyId, 4>::new();

        for ty_id in variant_tys {
            match &self.tys[ty_id] {
                TyKind::Variant(variant_ty) => {
                    non_variant_tys.extend(variant_ty.variant_tys.iter().cloned());
                }
                _ => non_variant_tys.push(ty_id),
            }
        }

        non_variant_tys.sort();
        non_variant_tys.dedup();

        if non_variant_tys.len() == 1 {
            return non_variant_tys[0];
        }

        self.add_ty(VariantTy {
            variant_tys: non_variant_tys,
        })
    }
}
