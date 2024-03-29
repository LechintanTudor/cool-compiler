use crate::{Decl, ParseResult, Parser};
use cool_lexer::{tk, TokenKind};
use cool_span::{Section, Span};
use derive_more::From;

#[derive(Clone, Debug)]
pub struct ModuleContent {
    pub decls: Vec<Decl>,
}

impl Section for ModuleContent {
    fn span(&self) -> Span {
        match (self.decls.first(), self.decls.last()) {
            (Some(first), Some(last)) => first.span.to(last.span),
            _ => Span::empty(),
        }
    }
}

#[derive(Clone, From, Debug)]
pub enum ModuleKind {
    External,
    Inline(ModuleContent),
}

#[derive(Clone, Debug)]
pub struct ModuleItem {
    pub span: Span,
    pub kind: ModuleKind,
}

impl Section for ModuleItem {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_module_item(&mut self) -> ParseResult<ModuleItem> {
        let start_token = self.bump_expect(&tk::KW_MODULE)?;

        let (kind, end_token) = if self.peek().kind == tk::OPEN_BRACE {
            self.bump();

            let mut decls = Vec::<Decl>::new();
            let end_token = loop {
                if self.peek().kind == tk::CLOSE_BRACE {
                    break self.bump();
                }

                decls.push(self.parse_decl()?);
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
        let mut decls = Vec::<Decl>::new();

        loop {
            if self.peek().kind == TokenKind::Eof {
                break;
            }

            let decl = self.parse_decl()?;
            decls.push(decl);
        }

        Ok(ModuleContent { decls })
    }
}
