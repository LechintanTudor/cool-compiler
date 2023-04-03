use crate::expr_ty::{ExprId, TyVarId};
use crate::resolve::BindingId;
use crate::ty::TyId;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Constraint {
    TyVar(TyVarId),
    Binding(BindingId),
    Expr(ExprId),
    Ty(TyId),
}

impl Constraint {
    #[inline]
    pub fn is_ty(&self) -> bool {
        matches!(self, Self::Ty(_))
    }

    #[inline]
    pub fn replace_if_eq(&mut self, compared: &Self, replacement: &Self) {
        if self == compared {
            *self = replacement.clone();
        }
    }
}

impl From<TyVarId> for Constraint {
    #[inline]
    fn from(ty_var_id: TyVarId) -> Self {
        Self::TyVar(ty_var_id)
    }
}

impl From<BindingId> for Constraint {
    #[inline]
    fn from(binding_id: BindingId) -> Self {
        Self::Binding(binding_id)
    }
}

impl From<ExprId> for Constraint {
    #[inline]
    fn from(expr_id: ExprId) -> Self {
        Self::Expr(expr_id)
    }
}

impl From<TyId> for Constraint {
    #[inline]
    fn from(ty_id: TyId) -> Self {
        Self::Ty(ty_id)
    }
}
