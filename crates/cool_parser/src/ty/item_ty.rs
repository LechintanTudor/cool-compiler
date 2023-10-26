use crate::{ParseResult, Parser};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::Span;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ItemKind {
    Alias,
    Module,
}

#[derive(Clone, Section, Debug)]
pub struct ItemTy {
    pub span: Span,
    pub kind: ItemKind,
}

impl Parser<'_> {
    pub fn parse_item_ty(&mut self) -> ParseResult<ItemTy> {
        let token = self.bump();

        let kind = match token.kind {
            tk::kw_alias => ItemKind::Alias,
            tk::kw_module => ItemKind::Module,
            _ => return self.error(token, &[tk::kw_alias, tk::kw_module]),
        };

        Ok(ItemTy {
            span: token.span,
            kind,
        })
    }
}
