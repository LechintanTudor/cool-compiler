use crate::{FnAst, ModuleAst};

#[derive(Clone, Debug)]
pub enum ItemAst {
    Module(ModuleAst),
    Fn(FnAst),
}
