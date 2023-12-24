use crate::{
    ArrayTy, FnAbi, FnTy, ItemId, ManyPtrTy, PtrTy, ResolveContext, SliceTy, StructTy, TupleTy,
    TyId, TyKind, VariantTy,
};
use std::collections::VecDeque;

impl ResolveContext {
    #[inline]
    pub fn add_ptr_ty(&mut self, pointee_ty: TyId, is_mutable: bool) -> TyId {
        self.add_ty(
            PtrTy {
                pointee_ty,
                is_mutable,
            }
            .into(),
        )
    }

    #[inline]
    pub fn add_many_ptr_ty(&mut self, pointee_ty: TyId, is_mutable: bool) -> TyId {
        self.add_ty(
            ManyPtrTy {
                pointee_ty,
                is_mutable,
            }
            .into(),
        )
    }

    #[inline]
    pub fn add_slice_ty(&mut self, elem_ty: TyId, is_mutable: bool) -> TyId {
        self.add_ty(
            SliceTy {
                elem_ty,
                is_mutable,
            }
            .into(),
        )
    }

    #[inline]
    pub fn add_array_ty(&mut self, elem_ty: TyId, len: u64) -> TyId {
        self.add_ty(ArrayTy { elem_ty, len }.into())
    }

    #[inline]
    pub fn add_tuple_ty(&mut self, elem_tys: &[TyId]) -> TyId {
        self.add_ty(
            TupleTy {
                elem_tys: elem_tys.into(),
            }
            .into(),
        )
    }

    #[inline]
    pub fn add_struct_ty(&mut self, item_id: ItemId) -> TyId {
        self.add_ty(StructTy { item_id }.into())
    }

    pub fn add_variant_ty(&mut self, variant_tys: &[TyId]) -> TyId {
        let mut all_variant_tys = Vec::<TyId>::new();

        let mut ty_queue = VecDeque::<TyId>::new();
        ty_queue.extend(variant_tys);

        while let Some(ty_id) = ty_queue.pop_front() {
            match &self.tys[ty_id] {
                TyKind::Variant(variant_ty) => ty_queue.extend(&variant_ty.variant_tys),
                _ => all_variant_tys.push(ty_id),
            }
        }

        all_variant_tys.sort();
        all_variant_tys.dedup();

        self.add_ty(
            VariantTy {
                variant_tys: all_variant_tys.into(),
            }
            .into(),
        )
    }

    pub fn add_fn_ty(
        &mut self,
        abi: FnAbi,
        param_tys: &[TyId],
        is_variadic: bool,
        return_ty: TyId,
    ) -> TyId {
        self.add_ty(
            FnTy {
                abi,
                param_tys: param_tys.into(),
                is_variadic,
                return_ty,
            }
            .into(),
        )
    }
}
