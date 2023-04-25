use crate::{AstGenerator, AstResult, ExprAst};
use cool_parser::DeclStmt;
use cool_resolve::{BindingId, FrameId};

pub struct DeclStmtAst {
    pub frame_id: FrameId,
    pub binding_id: BindingId,
    pub expr: ExprAst,
}

impl AstGenerator<'_> {
    pub fn gen_decl_stmt(
        &mut self,
        _current_frame_id: FrameId,
        _decl_stmt: &DeclStmt,
    ) -> AstResult<DeclStmtAst> {
        todo!()
    }
}
