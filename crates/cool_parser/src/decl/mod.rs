mod item_decl;
mod use_decl;

pub use self::item_decl::*;
pub use self::use_decl::*;
use crate::{ParseResult, Parser};
use cool_lexer::{tk, TokenKind};
use cool_span::{Section, Span};
use derive_more::From;

#[derive(Clone, Debug)]
pub struct Decl {
    pub span: Span,
    pub is_exported: bool,
    pub kind: DeclKind,
}

#[derive(Clone, From, Debug)]
pub enum DeclKind {
    Item(ItemDecl),
    Use(UseDecl),
}

impl DeclKind {
    #[inline]
    pub fn as_item_decl(&self) -> Option<&ItemDecl> {
        match self {
            Self::Item(item_decl) => Some(item_decl),
            _ => None,
        }
    }
}

impl Section for DeclKind {
    fn span(&self) -> Span {
        match self {
            Self::Item(decl) => decl.span(),
            Self::Use(decl) => decl.span(),
        }
    }
}

impl Parser<'_> {
    pub fn parse_decl(&mut self) -> ParseResult<Decl> {
        let export_span = if self.peek().kind == tk::KW_EXPORT {
            Some(self.bump().span)
        } else {
            None
        };

        let kind = match self.peek().kind {
            TokenKind::Ident(_) => DeclKind::Item(self.parse_item_decl()?),
            tk::KW_USE => DeclKind::Use(self.parse_use_decl()?),
            _ => self.peek_error(&[tk::KW_USE, tk::DIAG_IDENT])?,
        };

        let end_token = self.bump_expect(&tk::SEMICOLON)?;

        let (is_exported, span) = match export_span {
            Some(span) => (true, span.to(end_token.span)),
            None => (false, kind.span().to(end_token.span)),
        };

        Ok(Decl {
            span,
            is_exported,
            kind,
        })
    }
}
