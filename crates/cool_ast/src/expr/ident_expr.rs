use crate::expr::GenericExprAst;
use crate::{AstGenerator, AstResult, ResolveAst, TyMismatch};
use cool_parser::IdentExpr;
use cool_resolve::{BindingId, ExprId, ItemKind, ScopeId, TyId};

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
    fn resolve_exprs(&self, ast: &mut AstGenerator, expected_ty: TyId) -> AstResult<TyId> {
        let binding_ty = ast.resolve[self.binding_id].ty_id;

        let expr_ty = binding_ty
            .resolve_non_inferred(expected_ty)
            .ok_or(TyMismatch {
                found_ty: binding_ty,
                expected_ty,
            })?;

        ast.resolve.set_expr_ty(self.id, expr_ty);
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
                id: self.resolve.add_expr(),
                binding_id,
            },
            _ => todo!("return error"),
        }
    }
}
