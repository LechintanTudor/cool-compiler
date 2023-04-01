use crate::expr::GenericExprAst;
use crate::{AstGenerator, Unify};
use cool_parser::IdentExpr;
use cool_resolve::expr_ty::{Constraint, ExprId};
use cool_resolve::resolve::{BindingId, ScopeId, SymbolKind};

#[derive(Clone, Debug)]
pub struct IdentExprAst {
    pub id: ExprId,
    pub binding_id: BindingId,
}

impl Unify for IdentExprAst {
    fn unify(&self, gen: &mut AstGenerator) {
        gen.unification.add_constraint(
            Constraint::Expr(self.id),
            Constraint::Binding(self.binding_id),
        );
    }
}

impl GenericExprAst for IdentExprAst {
    #[inline]
    fn id(&self) -> ExprId {
        self.id
    }
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
            SymbolKind::Binding(binding_id) => IdentExprAst {
                id: self.unification.gen_expr(),
                binding_id,
            },
            _ => todo!("return error"),
        }
    }
}
