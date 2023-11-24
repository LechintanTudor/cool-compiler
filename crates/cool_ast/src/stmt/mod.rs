mod assign_stmt;
mod decl_stmt;

pub use self::assign_stmt::*;
pub use self::decl_stmt::*;

use crate::{AstGenerator, ExprAst, SpannedAstResult};
use cool_derive::Section;
use cool_parser::Stmt;
use cool_resolve::{tys, FrameId};
use derive_more::From;

#[derive(Clone, From, Section, Debug)]
pub enum StmtAst {
    Assign(AssignStmtAst),
    Decl(DeclStmtAst),
    Expr(ExprAst),
}

impl AstGenerator<'_> {
    pub fn gen_stmt(&mut self, stmt: &Stmt, frame_id: FrameId) -> SpannedAstResult<StmtAst> {
        let stmt = match stmt {
            Stmt::Assign(assign) => self.gen_assign_stmt(assign, frame_id)?.into(),
            Stmt::Decl(decl) => self.gen_decl_stmt(decl, frame_id)?.into(),
            Stmt::Expr(expr) => self.gen_expr(expr, frame_id, tys::infer)?.into(),
            _ => todo!(),
        };

        Ok(stmt)
    }
}
