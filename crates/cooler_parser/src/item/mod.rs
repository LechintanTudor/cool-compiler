mod module_item;
mod struct_item;

pub use self::module_item::*;
pub use self::struct_item::*;

use crate::{ExprId, Ident, ParseResult, Parser, TyId};
use cool_collections::define_index_newtype;
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::{Section, Span};
use derive_more::From;

define_index_newtype!(ItemId);

#[derive(Clone, Debug)]
pub enum Item {
    File(ModuleId),
    Inline(InlineItem),
}

impl Item {
    #[inline]
    #[must_use]
    pub fn kind(&self) -> ItemKind {
        match self {
            Self::File(module_id) => ItemKind::Module(*module_id),
            Self::Inline(item) => item.kind,
        }
    }
}

#[derive(Clone, Section, Debug)]
pub struct InlineItem {
    pub span: Span,
    pub ident: Ident,
    pub ty: Option<TyId>,
    pub kind: ItemKind,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, From, Debug)]
pub enum ItemKind {
    Alias(TyId),
    Expr(ExprId),
    Module(ModuleId),
    Struct(StructId),
}

impl Parser<'_> {
    pub fn parse_inline_item(&mut self) -> ParseResult<ItemId> {
        let ident = self.parse_ident()?;
        self.bump_expect(&tk::colon)?;

        let ty = (self.peek().kind != tk::colon)
            .then(|| self.parse_ty())
            .transpose()?;

        self.bump_expect(&tk::colon)?;

        let (kind, kind_span) = match self.peek().kind {
            tk::kw_alias => {
                let ty_id = self.parse_alias()?;
                let span = self.data.tys[ty_id].span();
                (ty_id.into(), span)
            }
            tk::kw_module => {
                let module_id = self.parse_module()?;
                let span = self.data.modules[module_id].span;
                (module_id.into(), span)
            }
            tk::kw_struct => {
                let struct_id = self.parse_struct()?;
                let span = self.data.structs[struct_id].span;
                (struct_id.into(), span)
            }
            _ => {
                let expr_id = self.parse_expr()?;
                let span = self.data.exprs[expr_id].span();
                (expr_id.into(), span)
            }
        };

        Ok(self.data.items.push(Item::Inline(InlineItem {
            span: ident.span.to(kind_span),
            ident,
            ty,
            kind,
        })))
    }

    fn parse_alias(&mut self) -> ParseResult<TyId> {
        self.bump_expect(&tk::kw_alias)?;
        self.parse_ty()
    }
}
