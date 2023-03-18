use crate::expr::GenericExprAst;
use crate::AstGenerator;
use cool_parser::IdentExpr;
use cool_resolve::binding::{BindingId, BindingTable, FrameId};
use cool_resolve::item::ItemId;
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

impl AstGenerator<'_> {
    pub fn generate_ident_expr(
        &mut self,
        _module_id: ItemId,
        parent_id: Option<FrameId>,
        expr: &IdentExpr,
    ) -> IdentExprAst {
        // TODO: Handle global consts

        IdentExprAst {
            binding_id: self
                .bindings
                .get_binding_id(parent_id.unwrap(), expr.ident.symbol)
                .unwrap(),
        }
    }
}
