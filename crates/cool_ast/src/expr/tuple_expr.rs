use crate::expr::{ExprAst, GenericExprAst};
use cool_resolve::expr_ty::ExprId;

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
