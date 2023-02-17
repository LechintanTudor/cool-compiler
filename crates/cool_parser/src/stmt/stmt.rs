use crate::error::ParseResult;
use crate::parser::Parser;
use crate::stmt::DeclStmt;
use crate::ParseTree;
use cool_lexer::tokens::Token;
use cool_span::Span;

#[derive(Clone, Debug)]
pub enum Stmt {
    Decl(DeclStmt),
}

impl ParseTree for Stmt {
    fn span(&self) -> Span {
        match self {
            Self::Decl(decl) => decl.span,
        }
    }
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn parse_stmt(&mut self) -> ParseResult<Stmt> {
        Ok(Stmt::Decl(self.parse_decl_stmt()?))
    }
}
