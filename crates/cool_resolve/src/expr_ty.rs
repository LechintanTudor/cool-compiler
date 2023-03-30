use crate::resolve::BindingId;
use crate::ty::TyId;
use cool_collections::{id_newtype, IdIndexedVec};
use rustc_hash::FxHashMap;
use std::collections::hash_map::Entry;

id_newtype!(ExprId);

#[derive(Default, Debug)]
pub struct ExprTyTable {
    exprs: IdIndexedVec<ExprId, TyId>,
    bindings: FxHashMap<BindingId, TyId>,
}

impl ExprTyTable {
    #[inline]
    pub fn add_expr(&mut self, ty_id: TyId) -> ExprId {
        self.exprs.push(ty_id)
    }

    #[inline]
    pub fn add_binding(&mut self, binding_id: BindingId, ty_id: TyId) {
        match self.bindings.entry(binding_id) {
            Entry::Vacant(entry) => entry.insert(ty_id),
            _ => todo!("handle incompatible types error"),
        };
    }

    #[inline]
    pub fn get_expr_ty_id(&self, expr_id: ExprId) -> TyId {
        self.exprs[expr_id]
    }

    #[inline]
    pub fn get_binding_ty_id(&self, binding_id: BindingId) -> TyId {
        *self.bindings.get(&binding_id).unwrap()
    }
}
