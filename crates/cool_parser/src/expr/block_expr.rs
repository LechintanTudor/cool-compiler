use crate::{BlockElem, Expr, ParseResult, Parser, Stmt};
use cool_lexer::tokens::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct BlockExpr {
    pub span: Span,
    pub stmts: Vec<Stmt>,
    pub expr: Option<Box<Expr>>,
}

impl Section for BlockExpr {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_block_expr(&mut self) -> ParseResult<BlockExpr> {
        let start_token = self.bump_expect(&tk::OPEN_BRACE)?;

        if let Some(end_token) = self.bump_if_eq(tk::CLOSE_BRACE) {
            return Ok(BlockExpr {
                span: start_token.span.to(end_token.span),
                stmts: vec![],
                expr: None,
            });
        }

        let mut stmts = Vec::<Stmt>::new();

        let (end_token, expr) = loop {
            match self.parse_block_elem()? {
                BlockElem::Expr(expr) => {
                    if let Some(end_token) = self.bump_if_eq(tk::CLOSE_BRACE) {
                        break (end_token, Some(expr));
                    }

                    if expr.is_promotable_to_stmt() {
                        stmts.push(Stmt::Expr(expr.into()));
                    } else {
                        self.peek_error(&[tk::SEMICOLON, tk::CLOSE_BRACE])?;
                    }
                }
                BlockElem::Stmt(stmt) => {
                    stmts.push(stmt);

                    if let Some(end_token) = self.bump_if_eq(tk::CLOSE_BRACE) {
                        break (end_token, None);
                    }
                }
            }
        };

        Ok(BlockExpr {
            span: start_token.span.to(end_token.span),
            stmts,
            expr: expr.map(Box::new),
        })
    }
}
