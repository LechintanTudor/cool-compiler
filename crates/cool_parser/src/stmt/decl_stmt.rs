use crate::expr::Expr;
use crate::ty::Ty;
use crate::{ParseResult, ParseTree, Parser, Pattern};
use cool_lexer::tokens::{tk, Token};
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct DeclStmt {
    pub span: Span,
    pub pattern: Pattern,
    pub ty: Option<Ty>,
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
        let _pattern = self.parse_pattern()?;
        self.bump_expect(&[tk::COLON])?;

        todo!()
    }
}
