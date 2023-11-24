use crate::{ResolveContext, TyId};
use cool_collections::define_index_newtype;
use std::ops::Index;

define_index_newtype!(ExprId);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ExprKind {
    Lvalue { is_mutable: bool },
    Rvalue,
}

impl ExprKind {
    #[inline]
    #[must_use]
    pub fn is_assignable(&self) -> bool {
        matches!(self, Self::Lvalue { is_mutable: true })
    }
}

#[derive(Clone, Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub ty_id: TyId,
}

impl ResolveContext<'_> {
    #[inline]
    pub fn add_expr(&mut self, expr: Expr) -> ExprId {
        self.exprs.push(expr)
    }

    pub fn add_rvalue_expr(&mut self, ty_id: TyId) -> ExprId {
        self.add_expr(Expr {
            kind: ExprKind::Rvalue,
            ty_id,
        })
    }

    pub fn add_lvalue_expr(&mut self, is_mutable: bool, ty_id: TyId) -> ExprId {
        self.add_expr(Expr {
            kind: ExprKind::Lvalue { is_mutable },
            ty_id,
        })
    }
}

impl Index<ExprId> for ResolveContext<'_> {
    type Output = Expr;

    #[inline]
    fn index(&self, expr_id: ExprId) -> &Self::Output {
        &self.exprs[expr_id]
    }
}
