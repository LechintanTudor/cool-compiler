use crate::{ResolveContext, TyId};
use cool_collections::id_newtype;
use std::ops;

id_newtype!(ExprId);

#[derive(Clone, Copy, Debug)]
pub enum ExprKind {
    Lvalue,
    Rvalue,
}

#[derive(Clone, Copy, Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub ty_id: TyId,
}

impl ResolveContext {
    #[inline]
    pub fn add_expr(&mut self, ty_id: TyId) -> ExprId {
        self.exprs.push(ty_id)
    }

    #[inline]
    pub fn set_expr_ty(&mut self, expr_id: ExprId, ty_id: TyId) {
        self.exprs[expr_id] = ty_id.resolve_non_inferred(self.exprs[expr_id]).unwrap();
    }
}

impl ops::Index<ExprId> for ResolveContext {
    type Output = TyId;

    #[inline]
    fn index(&self, expr_id: ExprId) -> &Self::Output {
        &self.exprs[expr_id]
    }
}
