use cool_resolve::expr_ty::ExprTyUnifier;
use cool_resolve::ty::TyTable;

pub trait Unify {
    fn unify(&self, unifier: &mut ExprTyUnifier, tys: &mut TyTable);
}
