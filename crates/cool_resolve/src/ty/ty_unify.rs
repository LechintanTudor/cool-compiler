use crate::{ResolveContext, ResolveError, ResolveResult, TyId, TyKind};

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
        if ty_id.is_definable() && (ty_id == expected_ty_id || ty_id.is_infer()) {
            return Ok((ty_id, UnificationMethod::Direct));
        }

        if ty_id.is_infer() && expected_ty_id.is_definable() {
            return Ok((expected_ty_id, UnificationMethod::Direct));
        }

        let expected_ty = &self.tys[ty_id];

        let error = Err(ResolveError::TysCannotBeUnified {
            ty_id,
            expected_ty_id,
        });

        let (ty_id, method) = match expected_ty {
            TyKind::Variant(variant_ty) => {
                if variant_ty
                    .variant_tys
                    .iter()
                    .any(|&variant_ty_id| variant_ty_id == ty_id)
                {
                    (ty_id, UnificationMethod::Wrap)
                } else {
                    return error;
                }
            }
            _ => return error,
        };

        Ok((ty_id, method))
    }
}
