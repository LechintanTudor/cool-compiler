use crate::{tys, ResolveContext, TyId};
use cool_collections::id_newtype;
use std::ops;

id_newtype!(ExprId);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ResolveExprKind {
    Lvalue { is_mutable: bool },
    Rvalue,
}

#[derive(Clone, Copy, Debug)]
pub struct ResolveExpr {
    pub ty_id: TyId,
    pub kind: ResolveExprKind,
}

impl ResolveExpr {
    #[inline]
    pub const fn lvalue(ty_id: TyId, is_mutable: bool) -> Self {
        Self {
            ty_id,
            kind: ResolveExprKind::Lvalue { is_mutable },
        }
    }

    #[inline]
    pub const fn rvalue(ty_id: TyId) -> Self {
        Self {
            ty_id,
            kind: ResolveExprKind::Rvalue,
        }
    }

    #[inline]
    pub const fn module() -> Self {
        Self {
            ty_id: tys::MODULE,
            kind: ResolveExprKind::Lvalue { is_mutable: false },
        }
    }

    #[inline]
    pub const fn ty() -> Self {
        Self {
            ty_id: tys::TY,
            kind: ResolveExprKind::Lvalue { is_mutable: false },
        }
    }

    #[inline]
    pub fn is_assignable(&self) -> bool {
        self.ty_id != tys::MODULE
            && self.ty_id != tys::TY
            && matches!(self.kind, ResolveExprKind::Lvalue { is_mutable: true })
    }

    #[inline]
    pub fn is_addressable(&self) -> bool {
        self.ty_id != tys::MODULE && self.ty_id != tys::TY
    }

    #[inline]
    pub fn is_mutably_addressable(&self) -> bool {
        self.ty_id != tys::MODULE
            && self.ty_id != tys::TY
            && !matches!(self.kind, ResolveExprKind::Lvalue { is_mutable: false })
    }
}

impl Default for ResolveExpr {
    fn default() -> Self {
        Self {
            ty_id: tys::UNIT,
            kind: ResolveExprKind::Rvalue,
        }
    }
}

impl ResolveContext {
    #[inline]
    pub fn add_expr(&mut self, expr: ResolveExpr) -> ExprId {
        self.exprs.push(expr)
    }

    #[inline]
    pub fn get_expr_ty_id(&self, expr_id: ExprId) -> TyId {
        self.exprs[expr_id].ty_id
    }
}

impl ops::Index<ExprId> for ResolveContext {
    type Output = ResolveExpr;

    #[inline]
    fn index(&self, expr_id: ExprId) -> &Self::Output {
        &self.exprs[expr_id]
    }
}
