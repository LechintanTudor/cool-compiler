use crate::error::ParseResult;
use crate::item::ItemDecl;
use crate::parser::Parser;
use crate::ParseTree;
use cool_lexer::tokens::{tk, Token, TokenKind};
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct ModuleContent {
    pub decls: Vec<ItemDecl>,
}

#[derive(Clone, Debug)]
pub struct ModuleItem {
    pub span: Span,
    pub content: ModuleContent,
}

impl ParseTree for ModuleItem {
    fn span(&self) -> Span {
        self.span
    }
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn parse_module_item(&mut self) -> ParseResult<ModuleItem> {
        let start_token = self.bump_expect(&[tk::KW_MODULE])?;
        self.bump_expect(&[tk::OPEN_BRACE])?;

        let mut decls = Vec::<ItemDecl>::new();
        let end_token = loop {
            if self.peek().kind == tk::CLOSE_BRACE {
                break self.bump();
            }

            decls.push(self.parse_item_decl()?);
        };

        Ok(ModuleItem {
            span: start_token.span.to(end_token.span),
            content: ModuleContent { decls },
        })
    }

    pub fn parse_module_file(&mut self) -> ParseResult<ModuleContent> {
        let mut decls = Vec::<ItemDecl>::new();

        loop {
            if self.peek().kind == TokenKind::Eof {
                break;
            }

            let decl = self.parse_item_decl()?;
            decls.push(decl);
        }

        Ok(ModuleContent { decls })
    }
}
