use crate::item::ItemDecl;
use crate::{ParseResult, ParseTree, Parser};
use cool_lexer::tokens::{tk, Token, TokenKind};
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct ModuleContent {
    pub decls: Vec<ItemDecl>,
}

#[derive(Clone, Debug)]
pub enum ModuleKind {
    External,
    Inline(ModuleContent),
}

#[derive(Clone, Debug)]
pub struct ModuleItem {
    pub span: Span,
    pub kind: ModuleKind,
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

        let (kind, end_token) = if self.peek().kind == tk::OPEN_BRACE {
            self.bump();

            let mut decls = Vec::<ItemDecl>::new();
            let end_token = loop {
                if self.peek().kind == tk::CLOSE_BRACE {
                    break self.bump();
                }

                decls.push(self.parse_item_decl()?);
            };

            (ModuleKind::Inline(ModuleContent { decls }), end_token)
        } else {
            (ModuleKind::External, start_token)
        };

        Ok(ModuleItem {
            span: start_token.span.to(end_token.span),
            kind,
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
