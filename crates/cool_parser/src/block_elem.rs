use crate::expr::Expr;
use crate::stmt::{ExprStmt, Stmt};
use crate::{ParseResult, ParseTree, Parser, UnexpectedToken};
use cool_lexer::tokens::{tk, Token, TokenKind};
use cool_span::Span;

#[derive(Clone, Debug)]
pub enum BlockElem {
    Expr(Expr),
    Stmt(Stmt),
}

impl ParseTree for BlockElem {
    #[inline]
    fn span(&self) -> Span {
        match self {
            Self::Expr(e) => e.span(),
            Self::Stmt(s) => s.span(),
        }
    }
}

impl From<Expr> for BlockElem {
    #[inline]
    fn from(expr: Expr) -> Self {
        Self::Expr(expr)
    }
}

impl From<Stmt> for BlockElem {
    #[inline]
    fn from(stmt: Stmt) -> Self {
        Self::Stmt(stmt)
    }
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn parse_block_elem(&mut self) -> ParseResult<BlockElem> {
        match self.peek().kind {
            TokenKind::Ident(_) => Ok(self.parse_expr_or_decl_or_assign()?),
            tk::KW_MUT => {
                let stmt = self.parse_decl_stmt()?;
                Ok(BlockElem::Stmt(Stmt::Decl(stmt)))
            }
            _ => self.parse_expr_or_expr_stmt(),
        }
    }

    fn parse_expr_or_expr_stmt(&mut self) -> ParseResult<BlockElem> {
        let expr = self.parse_expr()?;
        let elem = match self.bump_if_eq(tk::SEMICOLON) {
            Some(semicolon) => BlockElem::Stmt(Stmt::Expr(ExprStmt {
                span: expr.span().to(semicolon.span),
                expr,
            })),
            None => BlockElem::Expr(expr),
        };

        Ok(elem)
    }

    fn parse_expr_or_decl_or_assign(&mut self) -> ParseResult<BlockElem> {
        let expr = self.parse_expr()?;

        let elem = match expr {
            Expr::Ident(ident_expr) => match self.peek().kind {
                tk::COLON => {
                    let pattern = ident_expr.ident.into();
                    let stmt = self.continue_parse_decl_after_pattern(pattern)?;
                    BlockElem::Stmt(stmt.into())
                }
                tk::EQ => {
                    let pattern = ident_expr.ident.into();
                    let stmt = self.continue_parse_assign_after_pattern(pattern)?;
                    BlockElem::Stmt(stmt.into())
                }
                tk::SEMICOLON => {
                    let semicolon = self.bump_expect(&[tk::SEMICOLON])?;
                    BlockElem::Stmt(Stmt::Expr(ExprStmt {
                        span: ident_expr.span().to(semicolon.span),
                        expr: ident_expr.into(),
                    }))
                }
                tk::CLOSE_BRACE => BlockElem::Expr(ident_expr.into()),
                _ => Err(UnexpectedToken {
                    found: self.peek(),
                    expected: &[tk::COLON, tk::EQ, tk::SEMICOLON, tk::CLOSE_BRACE],
                })?,
            },
            expr => match self.peek().kind {
                tk::EQ => {
                    todo!("Add expression assignments and patterns");
                }
                tk::SEMICOLON => {
                    let semicolon = self.bump_expect(&[tk::SEMICOLON])?;
                    BlockElem::Stmt(Stmt::Expr(ExprStmt {
                        span: expr.span().to(semicolon.span),
                        expr: expr.into(),
                    }))
                }
                tk::CLOSE_BRACE => BlockElem::Expr(expr.into()),
                _ => Err(UnexpectedToken {
                    found: self.peek(),
                    expected: &[tk::EQ, tk::SEMICOLON, tk::CLOSE_BRACE],
                })?,
            },
        };

        Ok(elem)
    }
}
