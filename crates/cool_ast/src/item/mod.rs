use crate::{FnAst, ModuleAst};

#[derive(Clone, Debug)]
pub enum ItemAst {
    Fn(FnAst),
    Module(ModuleAst),
}

impl From<FnAst> for ItemAst {
    #[inline]
    fn from(fn_ast: FnAst) -> Self {
        Self::Fn(fn_ast)
    }
}

impl From<ModuleAst> for ItemAst {
    #[inline]
    fn from(module_ast: ModuleAst) -> Self {
        Self::Module(module_ast)
    }
}
