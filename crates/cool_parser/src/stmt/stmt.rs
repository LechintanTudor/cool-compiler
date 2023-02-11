use crate::error::ParseResult;
use crate::parser::Parser;
use crate::stmt::DeclStmt;
use cool_lexer::tokens::Token;

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
