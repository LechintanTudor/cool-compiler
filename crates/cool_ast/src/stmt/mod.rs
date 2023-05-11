mod assign_stmt;
mod decl_stmt;

pub use self::assign_stmt::*;
pub use self::decl_stmt::*;
use crate::{AstGenerator, AstResult, ExprAst};
use cool_parser::{Stmt, StmtKind};
use cool_resolve::{tys, FrameId};

#[derive(Clone, Debug)]
pub enum StmtAst {
    Assign(AssignStmtAst),
    Decl(DeclStmtAst),
    Expr(ExprAst),
}

impl AstGenerator<'_> {
    pub fn gen_stmt(&mut self, frame_id: FrameId, stmt: &Stmt) -> AstResult<StmtAst> {
        let stmt = match &stmt.kind {
            StmtKind::Assign(stmt) => StmtAst::Assign(self.gen_assign_stmt(frame_id, stmt)?),
            StmtKind::Decl(stmt) => StmtAst::Decl(self.gen_decl_stmt(frame_id, stmt)?),
            StmtKind::Defer(_) => todo!(),
            StmtKind::Expr(expr) => StmtAst::Expr(self.gen_expr(frame_id, tys::INFERRED, expr)?),
        };

        Ok(stmt)
    }
}
