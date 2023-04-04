use crate::expr::{ExprAst, GenericExprAst};
use crate::Unify;
use cool_resolve::expr_ty::{ExprId, ExprTyUnifier};
use cool_resolve::ty::TyTable;

pub struct TupleExprAst {
    pub id: ExprId,
    pub elems: Vec<ExprAst>,
}

impl GenericExprAst for TupleExprAst {
    #[inline]
    fn id(&self) -> ExprId {
        self.id
    }
}

impl Unify for TupleExprAst {
    fn unify(&self, unifier: &mut ExprTyUnifier, tys: &mut TyTable) {
        for elem in self.elems.iter() {
            elem.unify(unifier, tys);
        }

        unifier.solve_constraints();
    }
}
