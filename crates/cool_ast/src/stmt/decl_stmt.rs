use crate::{AstGenerator, AstResult, ExprAst};
use cool_parser::DeclStmt;
use cool_resolve::{BindingId, FrameId};

#[derive(Clone, Debug)]
pub struct DeclStmtAst {
    pub frame_id: FrameId,
    pub binding_id: BindingId,
    pub expr: Box<ExprAst>,
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
