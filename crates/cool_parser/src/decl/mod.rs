mod item_decl;
mod use_decl;

pub use self::item_decl::*;
pub use self::use_decl::*;

use crate::{ParseResult, Parser};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::{Section, Span};
use derive_more::From;

#[derive(Clone, Section, Debug)]
pub struct Decl {
    pub span: Span,
    pub is_exported: bool,
    pub kind: DeclKind,
}

#[derive(Clone, From, Section, Debug)]
pub enum DeclKind {
    Item(ItemDecl),
    Use(UseDecl),
}

impl Parser<'_> {
    pub fn parse_decl(&mut self) -> ParseResult<Decl> {
        let export_token = self.bump_if_eq(tk::kw_export);

        let kind: DeclKind = match self.peek().kind {
            tk::kw_use => self.parse_use_decl()?.into(),
            _ => self.parse_item_decl()?.into(),
        };

        let start_span = export_token
            .as_ref()
            .map(|token| token.span)
            .unwrap_or(kind.span());

        let semicolon = self.bump_expect(&tk::semicolon)?;

        Ok(Decl {
            span: start_span.to(semicolon.span),
            is_exported: export_token.is_some(),
            kind,
        })
    }
}
