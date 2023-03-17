use crate::expr::GenericExprAst;
use cool_resolve::binding::{BindingId, BindingTable};
use cool_resolve::ty::TyId;

#[derive(Clone, Debug)]
pub struct IdentExprAst {
    pub binding_id: BindingId,
}

impl GenericExprAst for IdentExprAst {
    fn ty_id(&self, bindings: &BindingTable) -> Option<TyId> {
        bindings.get_binding_by_id(self.binding_id).ty_id
    }
}
