mod assign_stmt;
mod decl_stmt;

pub use self::assign_stmt::*;
pub use self::decl_stmt::*;
use crate::{AstGenerator, AstResult, ExprAst};
use cool_parser::Stmt;
use cool_resolve::{tys, FrameId};

#[derive(Clone, Debug)]
pub enum StmtAst {
    Assign(AssignStmtAst),
    Decl(DeclStmtAst),
    Expr(ExprAst),
}

impl AstGenerator<'_> {
    pub fn gen_stmt(&mut self, frame_id: FrameId, stmt: &Stmt) -> AstResult<StmtAst> {
        let stmt = match stmt {
            Stmt::Assign(s) => StmtAst::Assign(self.gen_assign_stmt(frame_id, s)?),
            Stmt::Decl(s) => StmtAst::Decl(self.gen_decl_stmt(frame_id, s)?),
            Stmt::Expr(e) => StmtAst::Expr(self.gen_expr(frame_id, tys::INFERRED, &e.expr)?),
        };

        Ok(stmt)
    }
}
