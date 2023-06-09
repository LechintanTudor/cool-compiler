mod assign_stmt;
mod decl_stmt;
mod defer_stmt;
mod return_stmt;

pub use self::assign_stmt::*;
pub use self::decl_stmt::*;
pub use self::defer_stmt::*;
pub use self::return_stmt::*;
use crate::{AstGenerator, AstResult, ExprAst};
use cool_parser::{Stmt, StmtKind};
use cool_resolve::{tys, FrameId};
use cool_span::{Section, Span};
use derive_more::From;

#[derive(Clone, From, Debug)]
pub enum StmtAst {
    Assign(AssignStmtAst),
    Decl(DeclStmtAst),
    Defer(DeferStmtAst),
    Expr(ExprAst),
    Return(ReturnStmtAst),
}

impl StmtAst {
    pub fn get_new_frame_id(&self) -> Option<FrameId> {
        let frame_id = match self {
            Self::Decl(stmt) => stmt.frame_id,
            Self::Defer(stmt) => stmt.frame_id,
            _ => return None,
        };

        Some(frame_id)
    }

    #[inline]
    pub fn is_return(&self) -> bool {
        matches!(self, Self::Return(_))
    }
}

impl Section for StmtAst {
    fn span(&self) -> Span {
        match self {
            Self::Assign(stmt) => stmt.span(),
            Self::Decl(stmt) => stmt.span(),
            Self::Defer(stmt) => stmt.span(),
            Self::Expr(expr) => expr.span(),
            Self::Return(stmt) => stmt.span(),
        }
    }
}

impl AstGenerator<'_> {
    #[inline]
    pub fn gen_stmt(&mut self, frame_id: FrameId, stmt: &Stmt) -> AstResult<StmtAst> {
        self.gen_stmt_kind(frame_id, &stmt.kind)
    }

    pub fn gen_stmt_kind(&mut self, frame_id: FrameId, stmt_kind: &StmtKind) -> AstResult<StmtAst> {
        let stmt = match &stmt_kind {
            StmtKind::Assign(stmt) => self.gen_assign_stmt(frame_id, stmt)?.into(),
            StmtKind::Decl(stmt) => self.gen_decl_stmt(frame_id, stmt)?.into(),
            StmtKind::Defer(stmt) => self.gen_defer_stmt(frame_id, stmt)?.into(),
            StmtKind::Expr(expr) => self.gen_expr(frame_id, tys::INFER, expr)?.into(),
            StmtKind::Return(stmt) => self.gen_return_stmt(frame_id, stmt)?.into(),
        };

        Ok(stmt)
    }
}
