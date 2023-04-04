use crate::expr::{ExprAst, GenericExprAst};
use crate::{AstGenerator, ResolveAst, SemanticResult, Unify};
use cool_parser::ParenExpr;
use cool_resolve::expr_ty::{Constraint, ExprId, ExprTyUnifier};
use cool_resolve::resolve::ScopeId;
use cool_resolve::ty::{TyId, TyTable};

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

impl ResolveAst for ParenExprAst {
    fn resolve(&self, ast: &mut AstGenerator, expected_ty: Option<TyId>) -> SemanticResult<TyId> {
        let inner_ty = self.inner.resolve(ast, expected_ty)?;
        ast.expr_tys.set_expr_ty(self.id, inner_ty);
        Ok(inner_ty)
    }
}

impl AstGenerator<'_> {
    pub fn gen_paren_expr(&mut self, scope_id: ScopeId, expr: &ParenExpr) -> ParenExprAst {
        ParenExprAst {
            id: self.expr_tys.add_expr(),
            inner: Box::new(self.gen_expr(scope_id, &expr.inner)),
        }
    }
}
