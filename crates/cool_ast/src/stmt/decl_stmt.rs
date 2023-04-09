use crate::expr::ExprAst;
use crate::{AstGenerator, AstResult, ResolveAst, TyMismatch};
use cool_parser::{DeclStmt, Pattern};
use cool_resolve::{tys, BindingId, FrameId, ScopeId, TyId};

#[derive(Clone, Debug)]
pub struct DeclStmtAst {
    pub frame_id: FrameId,
    pub binding_id: BindingId,
    pub pattern: Pattern,
    pub explicit_ty_id: TyId,
    pub expr: ExprAst,
}

impl ResolveAst for DeclStmtAst {
    fn resolve_exprs(&self, ast: &mut AstGenerator, expected_ty: TyId) -> AstResult<TyId> {
        let expr_ty = self.expr.resolve_exprs(ast, self.explicit_ty_id)?;
        ast.resolve.set_binding_ty(self.binding_id, expr_ty);

        tys::UNIT
            .resolve_non_inferred(expected_ty)
            .ok_or(TyMismatch {
                found_ty: tys::UNIT,
                expected_ty,
            })?;

        Ok(tys::UNIT)
    }
}

impl AstGenerator<'_> {
    pub fn gen_decl_stmt(&mut self, scope_id: ScopeId, decl: &DeclStmt) -> DeclStmtAst {
        let frame_id = self.resolve.add_frame(scope_id);

        let explicit_ty_id = decl
            .ty
            .as_ref()
            .map(|ty| self.resolve_parsed_ty(scope_id, ty).unwrap())
            .unwrap_or(tys::INFERRED);

        let binding_id = self
            .resolve
            .insert_local_binding(frame_id, decl.pattern.is_mutable, decl.pattern.ident.symbol)
            .unwrap();

        let expr = self.gen_expr(scope_id, &decl.expr);

        DeclStmtAst {
            frame_id,
            binding_id,
            pattern: decl.pattern.clone(),
            explicit_ty_id,
            expr,
        }
    }
}
