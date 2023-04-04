use crate::{AstGenerator, SemanticResult};
use cool_resolve::expr_ty::ExprTyUnifier;
use cool_resolve::ty::{TyId, TyTable};

pub trait Unify {
    fn unify(&self, unifier: &mut ExprTyUnifier, tys: &mut TyTable);
}

pub trait ResolveAst {
    fn resolve(&self, ast: &mut AstGenerator, expected_ty: Option<TyId>) -> SemanticResult<TyId>;
}
