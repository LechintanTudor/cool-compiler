mod define_error;
mod resolve_alias;
mod resolve_binding;
mod resolve_error;
mod resolve_expr;
mod resolve_global;
mod resolve_local;
mod resolve_struct;
mod resolve_ty;

pub use self::define_error::*;
pub use self::resolve_alias::*;
pub use self::resolve_binding::*;
pub use self::resolve_error::*;
pub use self::resolve_expr::*;
pub use self::resolve_global::*;
pub use self::resolve_local::*;
pub use self::resolve_struct::*;
pub use self::resolve_ty::*;
use crate::{
    Binding, Frame, ItemKind, Module, PrimitiveTyData, TyConsts, TyContext, TyId, TyMismatch,
};
use cool_arena::SliceArena;
use cool_collections::IdIndexedVec;
use cool_lexer::Symbol;

#[derive(Debug)]
pub struct ResolveContext {
    paths: SliceArena<'static, ItemId, Symbol>,
    items: IdIndexedVec<ItemId, ItemKind>,
    modules: IdIndexedVec<ModuleId, Module>,
    tys: TyContext,
    bindings: IdIndexedVec<BindingId, Binding>,
    frames: IdIndexedVec<FrameId, Frame>,
    exprs: IdIndexedVec<ExprId, ResolveExpr>,
}

impl ResolveContext {
    pub fn new(primitives: PrimitiveTyData) -> Self {
        Self {
            paths: SliceArena::new_leak(),
            items: Default::default(),
            modules: Default::default(),
            tys: TyContext::new(primitives),
            bindings: Default::default(),
            frames: Default::default(),
            exprs: Default::default(),
        }
    }

    #[inline]
    pub fn ty_consts(&self) -> &TyConsts {
        self.tys.consts()
    }

    pub fn resolve_direct_ty_id(
        &self,
        found_ty_id: TyId,
        expected_ty_id: TyId,
    ) -> Result<TyId, TyMismatch> {
        self.resolve_direct_ty_id_inner(found_ty_id, expected_ty_id)
            .ok_or_else(|| {
                TyMismatch {
                    found_ty_id,
                    expected_ty_id,
                }
            })
    }

    fn resolve_direct_ty_id_inner(&self, found_ty_id: TyId, expected_ty_id: TyId) -> Option<TyId> {
        if found_ty_id.is_diverge() {
            return Some(expected_ty_id);
        }

        let tys = self.ty_consts();

        let ty_id = if expected_ty_id == tys.infer {
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
                || (found_ty_id == tys.infer_empty_array && expected_ty_id.is_array());

            if !can_resolve_directly {
                return None;
            }

            expected_ty_id
        };

        Some(ty_id)
    }
}
