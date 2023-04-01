mod decl_stmt;
mod expr_stmt;

pub use self::decl_stmt::*;
pub use self::expr_stmt::*;
use crate::{AstGenerator, Unify};
use cool_parser::Stmt;
use cool_resolve::resolve::ScopeId;

#[derive(Clone, Debug)]
pub enum StmtAst {
    Decl(DeclStmtAst),
    Expr(ExprStmtAst),
}

impl Unify for StmtAst {
    fn unify(&self, gen: &mut AstGenerator) {
        match self {
            Self::Decl(stmt) => stmt.unify(gen),
            Self::Expr(expr) => expr.unify(gen),
        }
    }
}

impl From<DeclStmtAst> for StmtAst {
    #[inline]
    fn from(stmt: DeclStmtAst) -> Self {
        Self::Decl(stmt)
    }
}

impl From<ExprStmtAst> for StmtAst {
    #[inline]
    fn from(stmt: ExprStmtAst) -> Self {
        Self::Expr(stmt)
    }
}

impl AstGenerator<'_> {
    pub fn gen_stmt(&mut self, scope_id: ScopeId, stmt: &Stmt) -> StmtAst {
        match stmt {
            Stmt::Decl(stmt) => self.gen_decl_stmt(scope_id, stmt).into(),
            _ => todo!(),
        }
    }
}
