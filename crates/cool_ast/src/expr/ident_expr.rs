use crate::AstGenerator;
use cool_parser::IdentExpr;
use cool_resolve::resolve::{BindingId, ScopeId, SymbolKind};

#[derive(Clone, Debug)]
pub struct IdentExprAst {
    pub binding_id: BindingId,
}

impl AstGenerator<'_> {
    pub fn gen_ident_expr(&mut self, scope_id: ScopeId, expr: &IdentExpr) -> IdentExprAst {
        let frame_id = match scope_id {
            ScopeId::Frame(frame_id) => frame_id,
            _ => todo!(),
        };

        let resolved = self
            .resolve
            .resolve_local(frame_id, expr.ident.symbol)
            .unwrap();

        match resolved {
            SymbolKind::Binding(binding_id) => IdentExprAst { binding_id },
            _ => todo!("return error"),
        }
    }
}
