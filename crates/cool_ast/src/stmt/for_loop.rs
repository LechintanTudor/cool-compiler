use crate::{AstGenerator, AstResult, BlockExprAst, DeclStmtAst, ExprAst, StmtAst};
use cool_parser::{ExprOrStmt, ForLoop};
use cool_resolve::FrameId;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct ForLoopAst {
    pub span: Span,
    pub decl: Box<DeclStmtAst>,
    pub cond: Box<ExprAst>,
    pub after: Box<StmtAst>,
    pub body: Box<BlockExprAst>,
}

impl Section for ForLoopAst {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl AstGenerator<'_> {
    pub fn gen_for_loop(&mut self, frame_id: FrameId, stmt: &ForLoop) -> AstResult<ForLoopAst> {
        let decl = self.gen_decl_stmt(frame_id, &stmt.decl)?;
        self.resolve.make_binding_mutable(decl.binding_id);
        let frame_id = decl.frame_id;

        let cond = self.gen_expr(frame_id, self.tys().bool, &stmt.cond)?;

        let after: StmtAst = match stmt.after.as_ref() {
            ExprOrStmt::Expr(expr) => self.gen_expr(frame_id, self.tys().infer, expr)?.into(),
            ExprOrStmt::Stmt(stmt) => self.gen_stmt_kind(frame_id, stmt)?,
        };

        let body = self.gen_block_expr(frame_id, self.tys().unit, &stmt.body)?;

        Ok(ForLoopAst {
            span: stmt.span,
            decl: Box::new(decl),
            cond: Box::new(cond),
            after: Box::new(after),
            body: Box::new(body),
        })
    }
}
