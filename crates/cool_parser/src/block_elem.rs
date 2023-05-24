use crate::expr::Expr;
use crate::stmt::Stmt;
use crate::{AssignOp, ParseResult, Parser, StmtKind};
use cool_lexer::tokens::tk;
use cool_span::{Section, Span};
use derive_more::From;

#[derive(Clone, From, Debug)]
pub enum BareBlockElem {
    Expr(Expr),
    Stmt(StmtKind),
}

impl Section for BareBlockElem {
    #[inline]
    fn span(&self) -> Span {
        match self {
            Self::Expr(expr) => expr.span(),
            Self::Stmt(stmt) => stmt.span(),
        }
    }
}

#[derive(Clone, From, Debug)]
pub enum BlockElem {
    Expr(Expr),
    Stmt(Stmt),
}

impl Section for BlockElem {
    #[inline]
    fn span(&self) -> Span {
        match self {
            Self::Expr(expr) => expr.span(),
            Self::Stmt(stmt) => stmt.span(),
        }
    }
}

impl Parser<'_> {
    #[inline]
    pub fn parse_block_elem(&mut self) -> ParseResult<BlockElem> {
        match self.parse_bare_block_elem(true, true)? {
            BareBlockElem::Expr(expr) => {
                match self.peek().kind {
                    tk::SEMICOLON => {
                        self.continue_parse_stmt(Box::new(expr).into())
                            .map(BlockElem::Stmt)
                    }
                    _ => Ok(BlockElem::Expr(expr)),
                }
            }
            BareBlockElem::Stmt(stmt) => self.continue_parse_stmt(stmt).map(BlockElem::Stmt),
        }
    }

    pub fn parse_bare_block_elem(
        &mut self,
        allow_defer_stmt: bool,
        allow_struct_expr: bool,
    ) -> ParseResult<BareBlockElem> {
        match self.peek().kind {
            tk::KW_MUT => {
                self.parse_decl_stmt()
                    .map(|decl_stmt| BareBlockElem::Stmt(decl_stmt.into()))
            }
            tk::KW_DEFER if allow_defer_stmt => {
                self.parse_defer_stmt()
                    .map(|defer_stmt| BareBlockElem::Stmt(defer_stmt.into()))
            }
            tk::KW_RETURN => {
                self.parse_return_stmt()
                    .map(|return_stmt| BareBlockElem::Stmt(return_stmt.into()))
            }
            _ => self.parse_expr_or_decl_or_assign(allow_struct_expr),
        }
    }

    fn parse_expr_or_decl_or_assign(
        &mut self,
        allow_struct_expr: bool,
    ) -> ParseResult<BareBlockElem> {
        let expr = self.parse_expr_full(allow_struct_expr)?;

        if let Expr::Ident(ident_expr) = &expr {
            if self.peek().kind == tk::COLON {
                return self
                    .continue_parse_decl(ident_expr.ident.into())
                    .map(|decl_stmt| BareBlockElem::Stmt(decl_stmt.into()));
            }
        }

        let elem = match self.peek().kind {
            tk::SEMICOLON => expr.into(),
            token => {
                match AssignOp::from_token_kind(token) {
                    Some(assign_op) => {
                        self.bump();

                        self.continue_parse_assign(expr, assign_op)
                            .map(|assign_stmt| BareBlockElem::Stmt(assign_stmt.into()))?
                    }
                    None => expr.into(),
                }
            }
        };

        Ok(elem)
    }
}
