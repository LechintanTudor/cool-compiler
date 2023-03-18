use crate::ItemAst;
use cool_lexer::symbols::Symbol;

#[derive(Clone, Debug)]
pub struct ItemDeclAst {
    pub symbol: Symbol,
    pub item: ItemAst,
}
