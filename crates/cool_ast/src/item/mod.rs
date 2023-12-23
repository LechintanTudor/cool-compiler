mod module_item;
mod struct_item;

pub use self::module_item::*;
pub use self::struct_item::*;

use crate::{ExprId, FnExprOrFnProto, FnProtoId, Ident, ParseResult, Parser, TyId};
use cool_collections::define_index_newtype;
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::{Section, Span};
use derive_more::From;

define_index_newtype!(ItemId);

#[derive(Clone, Section, Debug)]
pub struct Item {
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
    ExternFn(FnProtoId),
    ExternModule,
}

impl Parser<'_> {
    pub fn parse_item(&mut self) -> ParseResult<ItemId> {
        let ident = self.parse_ident()?;
        self.bump_expect(&tk::colon)?;

        let ty = (self.peek().kind != tk::colon)
            .then(|| self.parse_non_variant_ty())
            .transpose()?;

        self.bump_expect(&tk::colon)?;

        let (kind, kind_span) = match self.peek().kind {
            tk::kw_alias => {
                let ty_id = self.parse_alias()?;
                let span = self[ty_id].span();
                (ty_id.into(), span)
            }
            tk::kw_module => {
                let module_token = self.bump();

                if self.peek().kind == tk::open_brace {
                    let module_id = self.continue_parse_module(module_token)?;
                    let span = self[module_id].span;
                    (module_id.into(), span)
                } else {
                    (ItemKind::ExternModule, module_token.span)
                }
            }
            tk::kw_struct => {
                let struct_id = self.parse_struct()?;
                let span = self[struct_id].span;
                (struct_id.into(), span)
            }
            tk::kw_extern | tk::kw_fn => {
                match self.parse_fn_expr_or_fn_proto()? {
                    FnExprOrFnProto::Expr(expr_id) => {
                        let span = self[expr_id].span();
                        (expr_id.into(), span)
                    }
                    FnExprOrFnProto::Proto(proto_id) => {
                        let span = self[proto_id].span;
                        (proto_id.into(), span)
                    }
                }
            }
            _ => {
                let expr_id = self.parse_expr()?;
                let span = self[expr_id].span();
                (expr_id.into(), span)
            }
        };

        Ok(self.add_item(Item {
            span: ident.span.to(kind_span),
            ident,
            ty,
            kind,
        }))
    }

    fn parse_alias(&mut self) -> ParseResult<TyId> {
        self.bump_expect(&tk::kw_alias)?;
        self.parse_ty()
    }
}
