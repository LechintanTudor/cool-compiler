use crate::lexer::Token;
use crate::parser::{DeclStmt, ParseResult, Parser};

#[derive(Clone, Debug)]
pub enum Stmt {
    Decl(DeclStmt),
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn parse_stmt(&mut self) -> ParseResult<Stmt> {
        Ok(Stmt::Decl(self.parse_decl_stmt()?))
    }
}
