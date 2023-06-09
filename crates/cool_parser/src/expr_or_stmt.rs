use crate::expr::Expr;
use crate::{AssignOp, ParseResult, Parser, StmtKind};
use cool_lexer::tk;
use cool_span::{Section, Span};
use derive_more::From;

#[derive(Clone, From, Debug)]
pub enum ExprOrStmt {
    Expr(Expr),
    Stmt(StmtKind),
}

impl ExprOrStmt {
    #[inline]
    pub fn into_stmt_kind(self) -> StmtKind {
        match self {
            Self::Expr(expr) => StmtKind::Expr(Box::new(expr)),
            Self::Stmt(stmt) => stmt,
        }
    }
}

impl Section for ExprOrStmt {
    #[inline]
    fn span(&self) -> Span {
        match self {
            Self::Expr(expr) => expr.span(),
            Self::Stmt(stmt) => stmt.span(),
        }
    }
}

impl Parser<'_> {
    pub fn parse_bare_expr_or_stmt(
        &mut self,
        allow_defer_stmt: bool,
        allow_struct_expr: bool,
    ) -> ParseResult<ExprOrStmt> {
        match self.peek().kind {
            tk::KW_MUT => {
                self.parse_decl_stmt()
                    .map(|decl_stmt| ExprOrStmt::Stmt(decl_stmt.into()))
            }
            tk::KW_DEFER if allow_defer_stmt => {
                self.parse_defer_stmt()
                    .map(|defer_stmt| ExprOrStmt::Stmt(defer_stmt.into()))
            }
            tk::KW_RETURN => {
                self.parse_return_stmt()
                    .map(|return_stmt| ExprOrStmt::Stmt(return_stmt.into()))
            }
            _ => self.parse_expr_or_decl_or_assign(allow_struct_expr),
        }
    }

    fn parse_expr_or_decl_or_assign(&mut self, allow_struct_expr: bool) -> ParseResult<ExprOrStmt> {
        let expr = self.parse_expr_full(allow_struct_expr)?;

        if let Expr::Ident(ident_expr) = &expr {
            if self.peek().kind == tk::COLON {
                return self
                    .continue_parse_decl(ident_expr.ident.into())
                    .map(|decl_stmt| ExprOrStmt::Stmt(decl_stmt.into()));
            }
        }

        let elem = match self.peek().kind {
            tk::SEMICOLON => expr.into(),
            token => {
                match AssignOp::from_token_kind(token) {
                    Some(assign_op) => {
                        self.bump();

                        self.continue_parse_assign(expr, assign_op)
                            .map(|assign_stmt| ExprOrStmt::Stmt(assign_stmt.into()))?
                    }
                    None => expr.into(),
                }
            }
        };

        Ok(elem)
    }
}
