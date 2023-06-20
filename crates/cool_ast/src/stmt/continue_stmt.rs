use crate::{AstGenerator, AstResult};
use cool_parser::ContinueStmt;
use cool_resolve::FrameId;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct ContinueStmtAst {
    pub span: Span,
    pub frame_id: FrameId,
}

impl Section for ContinueStmtAst {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl AstGenerator<'_> {
    pub fn gen_continue_stmt(
        &mut self,
        frame_id: FrameId,
        stmt: &ContinueStmt,
    ) -> AstResult<ContinueStmtAst> {
        self.block_ty_id(stmt.span())?;

        Ok(ContinueStmtAst {
            span: stmt.span,
            frame_id,
        })
    }
}
