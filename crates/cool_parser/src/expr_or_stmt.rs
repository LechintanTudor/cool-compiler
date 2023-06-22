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

    #[inline]
    pub fn is_promotable_to_stmt(&self) -> bool {
        match self {
            Self::Expr(expr) => expr.is_promotable_to_stmt(),
            Self::Stmt(stmt) => stmt.is_promotable_to_stmt(),
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
            tk::KW_BREAK => {
                self.parse_break_stmt()
                    .map(StmtKind::from)
                    .map(ExprOrStmt::from)
            }
            tk::KW_CONTINUE => {
                self.parse_continue_stmt()
                    .map(StmtKind::from)
                    .map(ExprOrStmt::from)
            }
            tk::KW_DEFER if allow_defer_stmt => {
                self.parse_defer_stmt()
                    .map(StmtKind::from)
                    .map(ExprOrStmt::from)
            }
            tk::KW_FOR => {
                self.parse_for_loop()
                    .map(StmtKind::from)
                    .map(ExprOrStmt::from)
            }
            tk::KW_MUT => {
                self.parse_decl_stmt()
                    .map(StmtKind::from)
                    .map(ExprOrStmt::from)
            }
            tk::KW_RETURN => {
                self.parse_return_stmt()
                    .map(StmtKind::from)
                    .map(ExprOrStmt::from)
            }
            tk::KW_WHILE => {
                self.parse_while_loop()
                    .map(StmtKind::from)
                    .map(ExprOrStmt::from)
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
