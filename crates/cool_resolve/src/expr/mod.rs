use crate::{ResolveContext, TyId};
use cool_collections::define_index_newtype;
use std::ops::Index;

define_index_newtype!(ExprId);

#[derive(Clone, Debug)]
pub enum ExprKind {
    Lvalue { is_mutable: bool },
    Rvalue,
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
}

impl Index<ExprId> for ResolveContext<'_> {
    type Output = Expr;

    #[inline]
    fn index(&self, expr_id: ExprId) -> &Self::Output {
        &self.exprs[expr_id]
    }
}
