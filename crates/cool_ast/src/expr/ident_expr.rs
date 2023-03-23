use crate::AstGenerator;
use cool_parser::IdentExpr;
use cool_resolve::resolve::{BindingId, ScopeId};

#[derive(Clone, Debug)]
pub struct IdentExprAst {
    pub binding_id: BindingId,
}

impl AstGenerator<'_> {
    pub fn generate_ident_expr(&mut self, _scope_id: ScopeId, _expr: &IdentExpr) -> IdentExprAst {
        todo!()
    }
}
