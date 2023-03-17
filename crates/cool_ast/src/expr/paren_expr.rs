use crate::expr::{ExprAst, GenericExprAst};
use cool_resolve::binding::BindingTable;
use cool_resolve::ty::TyId;

#[derive(Clone, Debug)]
pub struct ParenExprAst {
    pub inner: Box<ExprAst>,
}

impl GenericExprAst for ParenExprAst {
    #[inline]
    fn ty_id(&self, bindings: &BindingTable) -> Option<TyId> {
        self.inner.ty_id(bindings)
    }
}
