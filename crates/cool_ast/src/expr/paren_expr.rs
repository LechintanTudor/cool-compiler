use crate::expr::{ExprAst, GenericExprAst};
use crate::{AstGenerator, Unify};
use cool_parser::ParenExpr;
use cool_resolve::expr_ty::{Constraint, ExprId, ExprTyUnifier};
use cool_resolve::resolve::ScopeId;
use cool_resolve::ty::TyTable;

#[derive(Clone, Debug)]
pub struct ParenExprAst {
    pub id: ExprId,
    pub inner: Box<ExprAst>,
}

impl Unify for ParenExprAst {
    fn unify(&self, unifier: &mut ExprTyUnifier, tys: &mut TyTable) {
        self.inner.unify(unifier, tys);

        unifier.add_constraint(Constraint::Expr(self.id), Constraint::Expr(self.inner.id()));
    }
}

impl GenericExprAst for ParenExprAst {
    #[inline]
    fn id(&self) -> ExprId {
        self.id
    }
}

impl AstGenerator<'_> {
    pub fn gen_paren_expr(&mut self, scope_id: ScopeId, expr: &ParenExpr) -> ParenExprAst {
        ParenExprAst {
            id: self.unification.add_expr(),
            inner: Box::new(self.gen_expr(scope_id, &expr.inner)),
        }
    }
}
