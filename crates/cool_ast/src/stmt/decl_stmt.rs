use crate::expr::ExprAst;
use crate::AstGenerator;
use cool_parser::DeclStmt;
use cool_resolve::binding::FrameId;
use cool_resolve::item::ItemId;
use cool_resolve::ty::TyId;

#[derive(Clone, Debug)]
pub struct DeclStmtAst {
    pub frame_id: FrameId,
    pub explicit_ty_id: Option<TyId>,
    pub expr: ExprAst,
}

impl AstGenerator<'_> {
    pub fn generate_decl_stmt(
        &mut self,
        module_id: ItemId,
        parent_id: Option<FrameId>,
        decl: &DeclStmt,
    ) -> DeclStmtAst {
        let frame_id = self.bindings.add_frame(module_id, parent_id);
        let explicit_ty_id = decl.ty.as_ref().map(|ty| self.resolve_ty(ty).unwrap());

        self.bindings
            .add_binding(
                frame_id,
                decl.pattern.ident.symbol,
                decl.pattern.is_mutable,
                explicit_ty_id,
            )
            .unwrap();

        let expr = self.generate_expr(module_id, parent_id, &decl.expr);

        DeclStmtAst {
            frame_id,
            explicit_ty_id,
            expr,
        }
    }
}
