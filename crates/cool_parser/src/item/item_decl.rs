use crate::item::Item;
use crate::{AbstractFn, ConstItem, Ident, ParseResult, ParseTree, Parser, Ty};
use cool_lexer::tokens::tk;
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct ItemDecl {
    pub ident: Ident,
    pub ty: Option<Ty>,
    pub item: Item,
}

impl ParseTree for ItemDecl {
    #[inline]
    fn span(&self) -> Span {
        self.ident.span.to(self.item.span())
    }
}

impl Parser<'_> {
    pub fn parse_item_decl(&mut self) -> ParseResult<ItemDecl> {
        let ident = self.parse_ident()?;
        self.bump_expect(&tk::COLON)?;

        let ty = if self.peek().kind != tk::COLON {
            Some(self.parse_ty()?)
        } else {
            None
        };

        self.bump_expect(&tk::COLON)?;

        let item: Item = match self.peek().kind {
            tk::KW_MODULE => self.parse_module_item()?.into(),
            tk::KW_FN | tk::KW_EXTERN => match self.parse_fn_or_extern_fn_item()? {
                AbstractFn::ExternFn(f) => f.into(),
                AbstractFn::Fn(f) => ConstItem { expr: f.into() }.into(),
            },
            _ => self.peek_error(&[tk::KW_MODULE, tk::KW_FN, tk::KW_EXTERN])?,
        };

        Ok(ItemDecl { ident, ty, item })
    }
}
