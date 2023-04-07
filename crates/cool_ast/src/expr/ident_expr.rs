use crate::expr::GenericExprAst;
use crate::{AstGenerator, ResolveAst, SemanticResult, TyMismatch};
use cool_parser::IdentExpr;
use cool_resolve::expr_ty::ExprId;
use cool_resolve::resolve::{BindingId, ItemKind, ScopeId};
use cool_resolve::ty::TyId;

#[derive(Clone, Debug)]
pub struct IdentExprAst {
    pub id: ExprId,
    pub binding_id: BindingId,
}

impl GenericExprAst for IdentExprAst {
    #[inline]
    fn id(&self) -> ExprId {
        self.id
    }
}

impl ResolveAst for IdentExprAst {
    fn resolve(&self, ast: &mut AstGenerator, expected_ty: TyId) -> SemanticResult<TyId> {
        let binding_ty = ast.expr_tys.get_binding_ty(self.binding_id);

        let expr_ty = binding_ty
            .resolve_non_inferred(expected_ty)
            .ok_or(TyMismatch {
                found_ty: binding_ty,
                expected_ty,
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
            ItemKind::Binding(binding_id) => IdentExprAst {
                id: self.expr_tys.add_expr(),
                binding_id,
            },
            _ => todo!("return error"),
        }
    }
}
