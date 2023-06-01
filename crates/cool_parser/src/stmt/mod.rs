mod assign_stmt;
mod decl_stmt;
mod defer_stmt;
mod return_stmt;

pub use self::assign_stmt::*;
pub use self::decl_stmt::*;
pub use self::defer_stmt::*;
pub use self::return_stmt::*;
use crate::{Expr, ParseResult, Parser};
use cool_lexer::tk;
use cool_span::{Section, Span};
use derive_more::From;

#[derive(Clone, From, Debug)]
pub enum StmtKind {
    Assign(AssignStmt),
    Decl(DeclStmt),
    Defer(DeferStmt),
    Expr(Box<Expr>),
    Return(ReturnStmt),
}

impl Section for StmtKind {
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

#[derive(Clone, Debug)]
pub struct Stmt {
    pub span: Span,
    pub kind: StmtKind,
}

impl Section for Stmt {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl From<StmtKind> for Stmt {
    #[inline]
    fn from(kind: StmtKind) -> Self {
        Self {
            span: kind.span(),
            kind,
        }
    }
}

impl Parser<'_> {
    pub fn continue_parse_stmt(&mut self, kind: StmtKind) -> ParseResult<Stmt> {
        if let Some(end_token) = self.bump_if_eq(tk::SEMICOLON) {
            return Ok(Stmt {
                span: kind.span().to(end_token.span),
                kind,
            });
        }

        if let StmtKind::Expr(expr) = &kind {
            if expr.is_promotable_to_stmt() {
                return Ok(Stmt {
                    span: expr.span(),
                    kind,
                });
            }
        }

        self.peek_error(&[tk::SEMICOLON])
    }
}
