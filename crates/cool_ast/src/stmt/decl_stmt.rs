use crate::expr::ExprAst;
use crate::AstGenerator;
use cool_parser::DeclStmt;
use cool_resolve::resolve::{FrameId, ScopeId};
use cool_resolve::ty::TyId;

#[derive(Clone, Debug)]
pub struct DeclStmtAst {
    pub frame_id: FrameId,
    pub explicit_ty_id: Option<TyId>,
    pub expr: ExprAst,
}

impl AstGenerator<'_> {
    pub fn generate_decl_stmt(&mut self, scope_id: ScopeId, decl: &DeclStmt) -> DeclStmtAst {
        let frame_id = self.resolve.insert_frame(scope_id);
        let explicit_ty_id = decl
            .ty
            .as_ref()
            .map(|ty| self.resolve_ty(scope_id, ty).unwrap());

        self.resolve
            .insert_local_binding(frame_id, decl.pattern.is_mutable, decl.pattern.ident.symbol)
            .unwrap();

        let expr = self.generate_expr(scope_id, &decl.expr);

        DeclStmtAst {
            frame_id,
            explicit_ty_id,
            expr,
        }
    }
}
