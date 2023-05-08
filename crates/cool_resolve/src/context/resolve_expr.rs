use crate::{tys, ResolveContext, TyId};
use cool_collections::id_newtype;
use std::ops;

id_newtype!(ExprId);

#[derive(Clone, Copy, Debug)]
pub struct ResolveExpr {
    pub ty_id: TyId,
    pub is_lvalue: bool,
}

impl ResolveExpr {
    #[inline]
    pub fn is_assignable(&self) -> bool {
        self.is_lvalue && self.ty_id != tys::MODULE
    }
}

impl Default for ResolveExpr {
    fn default() -> Self {
        Self {
            ty_id: tys::UNIT,
            is_lvalue: false,
        }
    }
}

impl ResolveContext {
    pub fn add_expr(&mut self, ty_id: TyId, is_lvalue: bool) -> ExprId {
        self.exprs.push(ResolveExpr { ty_id, is_lvalue })
    }

    #[inline]
    pub fn set_expr_ty(&mut self, expr_id: ExprId, ty_id: TyId) {
        let expr_ty_id = &mut self.exprs[expr_id].ty_id;
        *expr_ty_id = ty_id.resolve_non_inferred(*expr_ty_id).unwrap();
    }
}

impl ops::Index<ExprId> for ResolveContext {
    type Output = ResolveExpr;

    #[inline]
    fn index(&self, expr_id: ExprId) -> &Self::Output {
        &self.exprs[expr_id]
    }
}
