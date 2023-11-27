use crate::{DeclId, Item, ItemId, ParseResult, Parser};
use cool_collections::{define_index_newtype, SmallVec};
use cool_derive::Section;
use cool_lexer::tk;
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
    pub fn parse_file_module_item(&mut self) -> ParseResult<ItemId> {
        let mut decls = SmallVec::new();

        let span_end = loop {
            if self.peek().kind == tk::eof {
                break self.peek().span.end();
            }

            decls.push(self.parse_decl()?);
        };

        let module_id = self.data.modules.push(Module {
            span: Span::from_to(0, span_end),
            kind: ModuleKind::File,
            decls,
        });

        Ok(self.data.items.push(Item::File(module_id)))
    }

    pub fn parse_module(&mut self) -> ParseResult<ModuleId> {
        let module_token = self.bump_expect(&tk::kw_module)?;

        if self.bump_if_eq(tk::open_brace).is_none() {
            return Ok(self.data.modules.push(Module {
                span: module_token.span,
                kind: ModuleKind::File,
                decls: SmallVec::new(),
            }));
        }

        let mut decls = SmallVec::new();

        let end_span = loop {
            if let Some(close_brace) = self.bump_if_eq(tk::close_brace) {
                break close_brace.span;
            }

            decls.push(self.parse_decl()?);
        };

        Ok(self.data.modules.push(Module {
            span: module_token.span.to(end_span),
            kind: ModuleKind::Inline,
            decls,
        }))
    }
}
