use cool_lexer::symbols::Symbol;
use cool_resolve::ty::TyId;

#[derive(Clone, Debug)]
pub struct FnAst {
    pub ty: TyId,
    pub args: Vec<FnArgAst>,
}

#[derive(Clone, Debug)]
pub struct FnArgAst {
    pub is_mutable: bool,
    pub ident: Symbol,
}
