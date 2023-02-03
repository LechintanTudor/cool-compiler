use crate::lexer::{Token, TokenKind};
use crate::parser::{ParseResult, Parser, ItemDecl};
use crate::utils::Span;

#[derive(Clone, Debug)]
pub struct ModuleItem {
    pub span: Span,
    pub decls: Vec<ItemDecl>,
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn parse_module_item(&mut self) -> ParseResult<ModuleItem> {
        let mut decls = Vec::<ItemDecl>::new();

        let end = loop {
            if self.peek().kind == TokenKind::Eof {
                break self.bump().span.end();
            }

            let decl = self.parse_static_decl()?;
            decls.push(decl);
        };

        Ok(ModuleItem {
            span: Span::new(0, end),
            decls,
        })
    }
}
