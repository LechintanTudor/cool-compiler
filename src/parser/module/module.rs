use crate::lexer::{Token, TokenKind};
use crate::parser::{ParseResult, Parser, StaticDecl};
use crate::utils::Span;

#[derive(Clone, Debug)]
pub struct Module {
    pub span: Span,
    pub decls: Vec<StaticDecl>,
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn parse_module(&mut self) -> ParseResult<Module> {
        let mut decls = Vec::<StaticDecl>::new();

        let end = loop {
            if self.peek().kind == TokenKind::Eof {
                break self.bump().span.end();
            }

            let decl = self.parse_static_decl()?;
            decls.push(decl);
        };

        Ok(Module {
            span: Span::new(0, end),
            decls,
        })
    }
}
