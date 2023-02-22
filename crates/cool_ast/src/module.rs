use crate::ItemAst;

#[derive(Clone, Debug)]
pub struct ModuleAst {
    pub items: Vec<ItemAst>,
}
