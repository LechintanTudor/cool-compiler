use crate::lexer::Token;
use crate::parser2::root::RootAst;

pub struct Parser<T> {
    tokens: T,
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn new(tokens: T) -> Self {
        Self { tokens }
    }

    pub fn parse(&mut self) -> RootAst {
        todo!()
    }

    // pub fn bump()
}
