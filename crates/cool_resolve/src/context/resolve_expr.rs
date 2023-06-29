use crate::{ResolveContext, TyId};
use cool_arena::InternedValue;
use derive_more::Deref;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, Debug)]
#[deref(forward)]
pub struct ExprId(InternedValue<'static, ResolveExpr>);

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
    pub fn is_assignable(&self) -> bool {
        matches!(self.kind, ResolveExprKind::Lvalue { is_mutable: true })
    }

    #[inline]
    pub fn is_addressable(&self) -> bool {
        true
    }

    #[inline]
    pub fn is_mutably_addressable(&self) -> bool {
        !matches!(self.kind, ResolveExprKind::Lvalue { is_mutable: false })
    }
}

impl ResolveContext {
    #[inline]
    pub fn add_expr(&mut self, expr: ResolveExpr) -> ExprId {
        let expr: &'static _ = self.exprs.alloc(expr);
        ExprId(InternedValue::from(expr))
    }
}
