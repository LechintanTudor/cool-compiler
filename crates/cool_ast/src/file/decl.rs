use crate::{parse_error, ImportId, ItemId, ParseResult, Parser};
use cool_collections::define_index_newtype;
use cool_derive::Section;
use cool_lexer::{tk, TokenKind};
use cool_span::Span;
use derive_more::From;

define_index_newtype!(DeclId);

#[derive(Clone, Section, Debug)]
pub struct Decl {
    pub span: Span,
    pub is_exported: bool,
    pub kind: DeclKind,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, From, Debug)]
pub enum DeclKind {
    Item(ItemId),
    Import(ImportId),
}

impl Parser<'_> {
    pub fn parse_decl(&mut self) -> ParseResult<DeclId> {
        let export_token = self.bump_if_eq(tk::kw_export);
        let peeked_token = self.peek();

        let kind = match peeked_token.kind {
            tk::kw_use => self.parse_import_decl()?.into(),
            TokenKind::Ident(_) => self.parse_item()?.into(),
            _ => return parse_error(peeked_token, &[tk::kw_use, tk::identifier]),
        };

        let semicolon_token = self.bump_expect(&tk::semicolon)?;

        let start_span = export_token
            .map(|token| token.span)
            .unwrap_or(peeked_token.span);

        Ok(self.add_decl(Decl {
            span: start_span.to(semicolon_token.span),
            is_exported: export_token.is_some(),
            kind,
        }))
    }
}
