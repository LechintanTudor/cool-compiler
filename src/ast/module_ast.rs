use crate::ast::ItemAst;
use crate::symbol::Symbol;
use crate::utils::Span;

#[derive(Clone, Debug)]
pub struct ModuleAst {
    pub span: Span,
    pub name: Symbol,
    pub items: Vec<ItemAst>,
}
