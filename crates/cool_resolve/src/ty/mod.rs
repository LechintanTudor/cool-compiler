mod any_ty;
mod resolve_ty;
mod ty_error;
mod ty_id;
mod value_ty;

pub use self::any_ty::*;
pub use self::resolve_ty::*;
pub use self::ty_error::*;
pub use self::ty_id::*;
pub use self::value_ty::*;
use crate::tys;
use cool_arena::Arena;
use rustc_hash::FxHashMap;

#[derive(Debug)]
pub struct TyContext {
    primitives: PrimitiveTys,
    tys: Arena<'static, TyId, AnyTy>,
    resolve_tys: FxHashMap<TyId, ResolveTy>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AssignKind {
    Direct,
    MutPtrToPtr,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct AssignTy {
    pub ty_id: TyId,
    pub kind: AssignKind,
}

impl TyContext {
    pub fn new(primitives: PrimitiveTys) -> Self {
        Self {
            primitives,
            tys: Arena::new_leak(),
            resolve_tys: Default::default(),
        }
    }

    pub(crate) fn insert_builtin(&mut self, ty_id: TyId, ty: AnyTy) {
        self.tys.insert_checked(ty_id, ty.clone());

        if let AnyTy::Value(value) = ty {
            let resolve_ty = self.value_ty_to_resolve_ty(value).unwrap();
            self.resolve_tys.insert(ty_id, resolve_ty);
        }
    }

    pub fn get_or_insert(&mut self, ty: AnyTy) -> TyId {
        match ty {
            AnyTy::Value(value_ty) => {
                let ty_id = self.tys.get_or_insert(value_ty.clone().into());
                let resolve_ty = self.value_ty_to_resolve_ty(value_ty).unwrap();
                self.resolve_tys.insert(ty_id, resolve_ty);
                ty_id
            }
            _ => self.tys.get_or_insert(ty),
        }
    }

    #[inline]
    pub fn get_resolve_ty(&self, ty_id: TyId) -> Option<&ResolveTy> {
        self.resolve_tys.get(&ty_id)
    }

    #[inline]
    pub fn iter_resolve_ty_ids(&self) -> impl Iterator<Item = TyId> + '_ {
        self.resolve_tys.keys().copied()
    }

    pub fn resolve_direct_ty_id(&self, found_ty_id: TyId, expected_ty_id: TyId) -> Option<TyId> {
        if found_ty_id.is_divergent() {
            return Some(expected_ty_id);
        }

        match expected_ty_id {
            tys::INFER => {
                let ty_id = match found_ty_id {
                    tys::INFER_INT => tys::I32,
                    tys::INFER_FLOAT => tys::F64,
                    _ if !found_ty_id.is_inferred() => found_ty_id,
                    _ => return None,
                };

                Some(ty_id)
            }
            tys::INFER_NUMBER => {
                let ty_id = match found_ty_id {
                    tys::INFER_INT => tys::I32,
                    tys::INFER_FLOAT => tys::F64,
                    _ if found_ty_id.is_number() => found_ty_id,
                    _ => return None,
                };

                Some(ty_id)
            }
            tys::INFER_INT => {
                let ty_id = match found_ty_id {
                    tys::INFER_INT => tys::I32,
                    _ if found_ty_id.is_int() => found_ty_id,
                    _ => return None,
                };

                Some(ty_id)
            }
            tys::INFER_FLOAT => {
                let ty_id = match found_ty_id {
                    tys::INFER_INT => tys::F64,
                    tys::INFER_FLOAT => tys::F64,
                    _ if found_ty_id.is_float() => found_ty_id,
                    _ => return None,
                };

                Some(ty_id)
            }
            tys::INFER_SUBSCRIPT => {
                let ty_id = match found_ty_id {
                    tys::USIZE => tys::USIZE,
                    tys::RANGE_FULL => tys::RANGE_FULL,
                    tys::INFER_INT => tys::USIZE,
                    _ => return None,
                };

                Some(ty_id)
            }
            _ => {
                let can_resolve_directly = (found_ty_id == expected_ty_id)
                    || (found_ty_id == tys::INFER)
                    || (found_ty_id == tys::INFER_NUMBER && expected_ty_id.is_number())
                    || (found_ty_id == tys::INFER_INT && expected_ty_id.is_number())
                    || (found_ty_id == tys::INFER_FLOAT && expected_ty_id.is_float());

                if can_resolve_directly {
                    return Some(expected_ty_id);
                }

                match &self.get_resolve_ty(expected_ty_id)?.ty {
                    ValueTy::Array(_) => {
                        if found_ty_id == tys::INFER_EMPTY_ARRAY {
                            Some(expected_ty_id)
                        } else {
                            None
                        }
                    }
                    ValueTy::Ptr(pointer_ty) => {
                        let found_pointer_ty = self.get_resolve_ty(found_ty_id)?.ty.as_ptr()?;

                        let can_resolve = found_pointer_ty.pointee == pointer_ty.pointee
                            && !pointer_ty.is_mutable;

                        can_resolve.then_some(expected_ty_id)
                    }
                    _ => None,
                }
            }
        }
    }
}
