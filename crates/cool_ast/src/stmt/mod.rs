use crate::{AstGenerator, ExprAst, SpannedAstResult};
use cool_derive::Section;
use cool_parser::Stmt;
use cool_resolve::{tys, FrameId};

#[derive(Clone, Section, Debug)]
pub enum StmtAst {
    Expr(ExprAst),
}

impl AstGenerator<'_> {
    pub fn gen_stmt(&mut self, stmt: &Stmt, frame_id: FrameId) -> SpannedAstResult<StmtAst> {
        let stmt = match stmt {
            Stmt::Expr(expr) => {
                self.gen_expr(expr, frame_id, tys::infer)
                    .map(StmtAst::Expr)?
            }
            _ => todo!(),
        };

        Ok(stmt)
    }
}
