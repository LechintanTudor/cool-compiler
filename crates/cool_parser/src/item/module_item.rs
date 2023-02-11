use crate::error::ParseResult;
use crate::item::ItemDecl;
use crate::parser::Parser;
use cool_lexer::tokens::{Token, TokenKind};
use cool_span::Span;

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
