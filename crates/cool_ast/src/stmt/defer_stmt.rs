use crate::{AstGenerator, AstResult, StmtAst};
use cool_parser::DeferStmt;
use cool_resolve::FrameId;
use cool_span::{Section, Span};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct DeferStmtAst {
    pub span: Span,
    pub frame_id: FrameId,
    pub stmt: Arc<StmtAst>,
}

impl Section for DeferStmtAst {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl AstGenerator<'_> {
    pub fn gen_defer_stmt(
        &mut self,
        frame_id: FrameId,
        defer_stmt: &DeferStmt,
    ) -> AstResult<DeferStmtAst> {
        let frame_id = self.resolve.add_frame(frame_id.into());
        let stmt = Arc::new(self.gen_stmt(frame_id, &defer_stmt.stmt)?);
        self.defer_stmts.insert(frame_id, stmt.clone());

        Ok(DeferStmtAst {
            span: defer_stmt.span,
            frame_id,
            stmt,
        })
    }
}
