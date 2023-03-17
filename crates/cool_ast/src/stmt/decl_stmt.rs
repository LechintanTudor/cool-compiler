use crate::AstGenerator;
use crate::expr::ExprAst;
use cool_resolve::binding::FrameId;
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
    ) -> DeclStmtAst {
        todo!()
    }
}