use crate::ast::FnAst;
use crate::symbol::Symbol;
use crate::utils::Span;

#[derive(Clone, Debug)]
pub struct ItemAst {
    pub span: Span,
    pub is_exported: bool,
    pub ident_span: Span,
    pub ident: Symbol,
    pub kind: ItemKindAst,
}

#[derive(Clone, Debug)]
pub enum ItemKindAst {
    Fn(FnAst),
}
