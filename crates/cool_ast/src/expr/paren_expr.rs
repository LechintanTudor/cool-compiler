use crate::expr::{ExprAst, GenericExprAst};
use crate::AstGenerator;
use cool_parser::ParenExpr;
use cool_resolve::binding::{BindingTable, FrameId};
use cool_resolve::item::ItemId;
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

impl AstGenerator<'_> {
    pub fn generate_paren_expr(
        &mut self,
        module_id: ItemId,
        parent_id: Option<FrameId>,
        expr: &ParenExpr,
    ) -> ParenExprAst {
        ParenExprAst {
            inner: Box::new(self.generate_expr(module_id, parent_id, &expr.inner)),
        }
    }
}
