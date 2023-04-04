use crate::expr::GenericExprAst;
use crate::{AstGenerator, ResolveAst, SemanticResult, TyMismatch, Unify};
use cool_parser::IdentExpr;
use cool_resolve::expr_ty::{ExprId, ExprTyUnifier};
use cool_resolve::resolve::{BindingId, ScopeId, SymbolKind};
use cool_resolve::ty::{TyId, TyTable};

#[derive(Clone, Debug)]
pub struct IdentExprAst {
    pub id: ExprId,
    pub binding_id: BindingId,
}

impl Unify for IdentExprAst {
    fn unify(&self, unifier: &mut ExprTyUnifier, _tys: &mut TyTable) {
        unifier.add_constraint(self.id, self.binding_id);
    }
}

impl GenericExprAst for IdentExprAst {
    #[inline]
    fn id(&self) -> ExprId {
        self.id
    }
}

impl ResolveAst for IdentExprAst {
    fn resolve(
        &self,
        ast: &mut AstGenerator,
        expected_ty_id: Option<TyId>,
    ) -> SemanticResult<TyId> {
        let binding_ty = ast.expr_tys.get_binding_ty(self.binding_id);

        let expr_ty = binding_ty
            .resolve_non_inferred(expected_ty_id)
            .ok_or(TyMismatch {
                found_ty: binding_ty,
                expected_ty: expected_ty_id,
            })?;

        ast.expr_tys.set_expr_ty(self.id, expr_ty);
        Ok(expr_ty)
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
                id: self.expr_tys.add_expr(),
                binding_id,
            },
            _ => todo!("return error"),
        }
    }
}
