use crate::expr::ExprAst;
use crate::AstGenerator;
use cool_parser::{DeclStmt, Pattern};
use cool_resolve::resolve::{FrameId, ScopeId};
use cool_resolve::ty::TyId;

#[derive(Clone, Debug)]
pub struct DeclStmtAst {
    pub frame_id: FrameId,
    pub pattern: Pattern,
    pub explicit_ty_id: Option<TyId>,
    pub expr: ExprAst,
}

impl AstGenerator<'_> {
    pub fn gen_decl_stmt(&mut self, scope_id: ScopeId, decl: &DeclStmt) -> DeclStmtAst {
        let frame_id = self.resolve.insert_frame(scope_id);

        let explicit_ty_id = decl
            .ty
            .as_ref()
            .map(|ty| self.resolve_ty(scope_id, ty).unwrap());

        let binding_id = self
            .resolve
            .insert_local_binding(frame_id, decl.pattern.is_mutable, decl.pattern.ident.symbol)
            .unwrap();

        let expr = self.gen_expr(scope_id, &decl.expr);

        if let Some(explicit_ty_id) = explicit_ty_id {
            self.expr_tys.add_binding(binding_id, explicit_ty_id);
            assert_eq!(explicit_ty_id, self.expr_tys.get_expr_ty_id(expr.id));
        } else {
            let expr_ty = self.expr_tys.get_expr_ty_id(expr.id);
            self.expr_tys.add_binding(binding_id, expr_ty);
        }

        DeclStmtAst {
            frame_id,
            pattern: decl.pattern.clone(),
            explicit_ty_id,
            expr,
        }
    }
}
