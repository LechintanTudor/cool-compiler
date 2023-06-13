use crate::{ParseResult, Parser};
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ItemKind {
    Module,
    Ty,
}

#[derive(Clone, Debug)]
pub struct ItemTy {
    pub span: Span,
    pub kind: ItemKind,
}

impl Section for ItemTy {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_item_ty(&mut self) -> ParseResult<ItemTy> {
        let token = self.bump();

        let kind = match token.kind {
            tk::KW_MODULE => ItemKind::Module,
            tk::KW_TYPE => ItemKind::Ty,
            _ => self.error(token, &[tk::KW_MODULE, tk::KW_TYPE])?,
        };

        Ok(ItemTy {
            span: token.span,
            kind,
        })
    }
}
