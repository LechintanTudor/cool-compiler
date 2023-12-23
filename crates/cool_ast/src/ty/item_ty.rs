use crate::{parse_error, ParseResult, Parser, TyId};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::Span;

#[derive(Clone, Section, Debug)]
pub struct ItemTy {
    pub span: Span,
    pub kind: ItemTyKind,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ItemTyKind {
    Alias,
    Module,
}

impl Parser<'_> {
    pub fn parse_item_ty(&mut self) -> ParseResult<TyId> {
        let token = self.bump();

        let kind = match token.kind {
            tk::kw_alias => ItemTyKind::Alias,
            tk::kw_module => ItemTyKind::Module,
            _ => return parse_error(token, &[tk::kw_alias, tk::kw_module]),
        };

        Ok(self.add_ty(ItemTy {
            span: token.span,
            kind,
        }))
    }
}
