use crate::ast::ModuleAst;

#[derive(Clone, Debug)]
pub enum ItemAst {
    Module(ModuleAst),
}
