use cool_arena::{SliceArena, SliceHandle};
use cool_lexer::symbols::Symbol;

// #[derive(Clone, Debug)]
// pub struct Module {
//     pub symbol: Symbol,
//     pub children: Vec<ItemId>,
// }

// #[derive(Clone, Copy, Debug)]
// pub struct Item {
//     pub symbol: Symbol,
//     pub id: ItemId,
// }

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ItemId(SliceHandle<Symbol>);

#[derive(Default, Debug)]
pub struct ItemTable {
    paths: SliceArena<Symbol>,
}

impl ItemTable {
    #[inline]
    pub fn insert(&mut self, path: &[Symbol]) -> ItemId {
        ItemId(self.paths.insert(path))
    }

    #[inline]
    pub fn get(&self, item: ItemId) -> &[Symbol] {
        self.paths.get(item.0)
    }
}
