use crate::lexer::Token;
use crate::parser::error::ParseResult;
use crate::parser::parser::Parser;

#[derive(Clone, Debug)]
pub struct IdentAst {
    pub index: u32,
}

// A list of dot separated identifiers.
#[derive(Clone, Debug)]
pub struct PathAst {
    pub ident_indexes: Vec<u32>,
}

#[derive(Clone, Debug)]
pub enum IdentOrPathAst {
    Ident(IdentAst),
    Path(PathAst),
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn ident_or_path_ast(&mut self) -> ParseResult<IdentOrPathAst> {
        todo!()
    }
}
