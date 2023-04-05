use crate::resolve::BindingId;
use crate::ty::{tys, TyId};
use cool_collections::{id_newtype, IdIndexedVec};
use rustc_hash::FxHashMap;

id_newtype!(ExprId);

#[derive(Default, Debug)]
pub struct ExprTyTable {
    expr_tys: IdIndexedVec<ExprId, TyId>,
    binding_tys: FxHashMap<BindingId, TyId>,
}

impl ExprTyTable {
    #[inline]
    pub fn add_expr(&mut self) -> ExprId {
        self.expr_tys.push(tys::INFERRED)
    }

    pub fn set_expr_ty(&mut self, expr_id: ExprId, ty_id: TyId) {
        let expr_ty_id = &mut self.expr_tys[expr_id];
        *expr_ty_id = expr_ty_id.resolve_non_inferred(ty_id).unwrap();
    }

    pub fn set_binding_ty(&mut self, binding_id: BindingId, ty_id: TyId) {
        assert!(self.binding_tys.insert(binding_id, ty_id).is_none())
    }

    #[inline]
    pub fn get_expr_ty(&self, expr_id: ExprId) -> TyId {
        self.expr_tys[expr_id]
    }

    #[inline]
    pub fn get_binding_ty(&self, binding_id: BindingId) -> TyId {
        self.binding_tys[&binding_id]
    }
}
