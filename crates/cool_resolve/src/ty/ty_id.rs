use crate::{TyContext, TyShape};
use cool_arena::InternedValue;
use derive_more::{Deref, Display, From};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, From, Deref, Display, Debug)]
#[deref(forward)]
pub struct TyId(InternedValue<'static, TyShape>);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TyResolutionMethod {
    Direct,
    DropConst,
    WrapInVariant { wrapped_ty_id: TyId },
}

impl TyContext {
    #[allow(clippy::if_same_then_else)]
    pub fn resolve_ty_id(
        &self,
        found_ty_id: TyId,
        expected_ty_id: TyId,
    ) -> Option<(TyId, TyResolutionMethod)> {
        let tys = &self.consts;

        if found_ty_id.is_diverge() && expected_ty_id.is_value() {
            return Some((expected_ty_id, TyResolutionMethod::Direct));
        }

        if expected_ty_id
            .as_variant()
            .and_then(|variant| variant.get_variant_index(found_ty_id))
            .is_some()
        {
            return Some((
                expected_ty_id,
                TyResolutionMethod::WrapInVariant {
                    wrapped_ty_id: found_ty_id,
                },
            ));
        }

        if let Some(expected_pointee) = expected_ty_id
            .as_ptr()
            .filter(|ptr| ptr.is_mutable)
            .map(|ptr| ptr.pointee)
        {
            if found_ty_id
                .as_ptr()
                .filter(|ptr| !ptr.is_mutable)
                .is_some_and(|ptr| ptr.pointee == expected_pointee)
            {
                return Some((expected_ty_id, TyResolutionMethod::DropConst));
            }
        }

        if let Some(expected_pointee) = expected_ty_id
            .as_many_ptr()
            .filter(|many_ptr| many_ptr.is_mutable)
            .map(|many_ptr| many_ptr.pointee)
        {
            if found_ty_id
                .as_many_ptr()
                .filter(|many_ptr| !many_ptr.is_mutable)
                .is_some_and(|many_ptr| many_ptr.pointee == expected_pointee)
            {
                return Some((expected_ty_id, TyResolutionMethod::DropConst));
            }
        }

        if let Some(expected_elem) = expected_ty_id
            .as_slice()
            .filter(|slice| slice.is_mutable)
            .map(|slice| slice.elem)
        {
            if found_ty_id
                .as_slice()
                .filter(|slice| !slice.is_mutable)
                .is_some_and(|slice| slice.elem == expected_elem)
            {
                return Some((expected_ty_id, TyResolutionMethod::DropConst));
            }
        }

        let direct_ty_id = if expected_ty_id == tys.infer {
            if found_ty_id == tys.infer_int {
                tys.i32
            } else if found_ty_id == tys.infer_float {
                tys.f64
            } else if !found_ty_id.is_infer() {
                found_ty_id
            } else {
                return None;
            }
        } else if expected_ty_id == tys.infer_number {
            if found_ty_id == tys.infer_int {
                tys.i32
            } else if found_ty_id == tys.infer_float {
                tys.f32
            } else if found_ty_id.is_number() {
                found_ty_id
            } else {
                return None;
            }
        } else if expected_ty_id == tys.infer_int {
            if found_ty_id == tys.infer_int {
                tys.i32
            } else if found_ty_id.is_int() {
                found_ty_id
            } else {
                return None;
            }
        } else if expected_ty_id == tys.infer_float {
            if found_ty_id == tys.infer_int {
                tys.f64
            } else if found_ty_id == tys.infer_float {
                tys.f64
            } else if found_ty_id.is_float() {
                found_ty_id
            } else {
                return None;
            }
        } else {
            let can_resolve_directly = (found_ty_id == expected_ty_id)
                || (found_ty_id == tys.infer)
                || (found_ty_id == tys.infer_number && expected_ty_id.is_number())
                || (found_ty_id == tys.infer_int && expected_ty_id.is_number())
                || (found_ty_id == tys.infer_float && expected_ty_id.is_float())
                || (found_ty_id == tys.infer_array && expected_ty_id.is_array());

            if !can_resolve_directly {
                return None;
            }

            expected_ty_id
        };

        Some((direct_ty_id, TyResolutionMethod::Direct))
    }
}
