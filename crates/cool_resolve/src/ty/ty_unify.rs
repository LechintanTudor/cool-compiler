use crate::{tys, ResolveContext, ResolveError, ResolveResult, TyId};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum UnificationMethod {
    Direct,
    Wrap,
}

impl ResolveContext<'_> {
    pub fn unify_tys(
        &self,
        ty_id: TyId,
        expected_ty_id: TyId,
    ) -> ResolveResult<(TyId, UnificationMethod)> {
        let can_unify_directly = match expected_ty_id {
            tys::infer => ty_id.is_definable(),
            tys::infer_number => ty_id.is_number(),
            tys::infer_int_or_bool => ty_id.is_int() || ty_id == tys::bool,
            _ => ty_id == expected_ty_id,
        };

        if can_unify_directly {
            return Ok((ty_id, UnificationMethod::Direct));
        }

        self.tys[ty_id]
            .try_as_variant()
            .is_some_and(|variant_ty| {
                variant_ty
                    .variant_tys
                    .iter()
                    .any(|&variant_ty_id| variant_ty_id == ty_id)
            })
            .then_some((ty_id, UnificationMethod::Wrap))
            .ok_or(ResolveError::TysCannotBeUnified {
                ty_id,
                expected_ty_id,
            })
    }
}
