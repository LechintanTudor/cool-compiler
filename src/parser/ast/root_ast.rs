use crate::parser::ast::FnAst;

#[derive(Clone, Debug)]
pub struct RootAst {
    pub fns: Vec<FnAst>,
}
