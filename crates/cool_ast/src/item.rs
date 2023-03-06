use crate::FnAst;

#[derive(Clone, Debug)]
pub enum ItemAst {
    Fn(FnAst),
}
