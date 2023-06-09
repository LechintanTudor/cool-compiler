use crate::{Expr, ParseResult, Parser, Stmt};
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct DeferStmt {
    pub span: Span,
    pub stmt: Box<Stmt>,
}

impl Section for DeferStmt {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_defer_stmt(&mut self) -> ParseResult<DeferStmt> {
        let start_token = self.bump_expect(&tk::KW_DEFER)?;
        let expr_or_stmt = self.parse_bare_expr_or_stmt(false, false)?;

        let stmt = match self.bump_if_eq(tk::SEMICOLON) {
            Some(end_token) => {
                let span = expr_or_stmt.span().to(end_token.span);
                let kind = expr_or_stmt.into_stmt_kind();
                Stmt { span, kind }
            }
            None => {
                let span = start_token.span.to(expr_or_stmt.span());
                let kind = expr_or_stmt.into_stmt_kind();

                assert!(kind.as_expr().is_some_and(Expr::is_promotable_to_stmt));
                Stmt { span, kind }
            }
        };

        Ok(DeferStmt {
            span: start_token.span.to(stmt.span()),
            stmt: Box::new(stmt),
        })
    }
}
