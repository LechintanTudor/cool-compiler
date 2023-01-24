use crate::lexer::Token;
use crate::parser::Parser;
use crate::symbol::Symbol;
use crate::utils::Span;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub enum Mutable {
    #[default]
    No,
    Yes,
}

pub struct DeclStmtAst {
    pub span: Span,
    pub mutable: Mutable,
    pub type_ast: TypeAst,
    pub expr_ast: LiteralExprAst,
}

pub struct TypeAst {
    pub span: Span,
    pub ident: Symbol,
}

pub struct LiteralExprAst {
    pub span: Span,
    pub symbol: Symbol,
}

impl<T> Parser<T> where T: Iterator<Item = Token> {}
