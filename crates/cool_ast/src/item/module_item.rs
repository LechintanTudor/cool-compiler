use crate::{DeclId, ParseResult, Parser};
use cool_collections::{define_index_newtype, SmallVec};
use cool_derive::Section;
use cool_lexer::{tk, Token};
use cool_span::Span;

define_index_newtype!(ModuleId);

#[derive(Clone, Section, Debug)]
pub struct Module {
    pub span: Span,
    pub kind: ModuleKind,
    pub decls: SmallVec<DeclId, 4>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ModuleKind {
    File,
    Inline,
}

impl Parser<'_> {
    pub fn parse_file_module(&mut self) -> ParseResult<ModuleId> {
        let mut decls = SmallVec::new();

        let span_end = loop {
            if self.peek().kind == tk::eof {
                break self.peek().span.end();
            }

            decls.push(self.parse_decl()?);
        };

        Ok(self.add_module(Module {
            span: Span::from_to(0, span_end),
            kind: ModuleKind::File,
            decls,
        }))
    }

    pub fn continue_parse_module(&mut self, module_token: Token) -> ParseResult<ModuleId> {
        debug_assert_eq!(module_token.kind, tk::kw_module);

        self.bump_expect(&tk::open_brace)?;
        let mut decls = SmallVec::new();

        let end_span = loop {
            if let Some(close_brace) = self.bump_if_eq(tk::close_brace) {
                break close_brace.span;
            }

            decls.push(self.parse_decl()?);
        };

        Ok(self.add_module(Module {
            span: module_token.span.to(end_span),
            kind: ModuleKind::Inline,
            decls,
        }))
    }
}
