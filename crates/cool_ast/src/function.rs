use crate::BlockExprAst;
use cool_resolve::ItemId;

#[derive(Clone, Debug)]
pub struct FnAst {
    pub item_id: ItemId,
    pub body: BlockExprAst,
}

#[derive(Clone, Debug)]
pub struct ExternFnAst {
    pub item_id: ItemId,
}
