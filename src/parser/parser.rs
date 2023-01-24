use crate::lexer::Token;
use crate::parser::root::RootAst;

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

    pub fn bump(&mut self) -> Token {
        self.tokens.next().unwrap_or(Token::eof(0))
    }

    pub fn parse(&mut self) -> RootAst {
        todo!()
    }
}
