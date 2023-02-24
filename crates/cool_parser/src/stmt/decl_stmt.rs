use crate::expr::Expr;
use crate::{Ident, ParseResult, ParseTree, Parser};
use cool_lexer::tokens::{tk, Token};
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct DeclStmt {
    pub span: Span,
    pub is_mutable: bool,
    pub ident: Ident,
    pub expr: Expr,
}

impl ParseTree for DeclStmt {
    fn span(&self) -> Span {
        self.span
    }
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn parse_decl_stmt(&mut self) -> ParseResult<DeclStmt> {
        let start_token = self.bump();

        let is_mutable = match self.peek().kind {
            tk::KW_MUT => {
                self.bump();
                true
            }
            _ => false,
        };

        let ident = self.parse_ident()?;

        self.bump_expect(&[tk::COLON])?;
        self.bump_expect(&[tk::EQ])?;

        let expr = self.parse_expr()?;
        let semi = self.bump_expect(&[tk::SEMICOLON])?;

        Ok(DeclStmt {
            span: start_token.span.to(semi.span),
            is_mutable,
            ident,
            expr,
        })
    }
}
