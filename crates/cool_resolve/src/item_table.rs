use crate::item_path::{ItemPath, ItemPathBuf};
use cool_arena::{SliceArena, SliceHandle};
use cool_lexer::symbols::Symbol;
use rustc_hash::FxHashMap;

#[derive(Clone, Debug)]
pub struct Module {
    pub symbol: Symbol,
    pub children: FxHashMap<Symbol, Item>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Item(SliceHandle<Symbol>);

#[derive(Default, Debug)]
pub struct ItemTable {
    paths: SliceArena<Symbol>,
    modules: FxHashMap<Item, Module>,
}

impl ItemTable {
    #[inline]
    pub fn build_module(&mut self, path: ItemPathBuf) -> ModuleBuilder {
        ModuleBuilder::new(self, path)
    }

    #[inline]
    fn insert_if_not_exists<'a, P>(&mut self, path: P) -> Option<Item>
    where
        P: Into<ItemPath<'a>>,
    {
        self.paths
            .insert_if_not_exists(path.into().as_symbol_slice())
            .map(Item)
    }

    #[inline]
    pub fn get_item<'a, P>(&self, path: P) -> Option<Item>
    where
        P: Into<ItemPath<'a>>,
    {
        self.paths
            .get_handle(path.into().as_symbol_slice())
            .map(Item)
    }

    #[inline]
    pub fn get(&self, item: Item) -> ItemPath {
        self.paths.get(item.0).into()
    }

    #[inline]
    pub fn iter_paths(&self) -> impl Iterator<Item = ItemPath> {
        self.paths.iter().map(ItemPath::from)
    }
}

#[derive(Debug)]
pub struct ModuleBuilder<'a> {
    items: &'a mut ItemTable,
    item: Item,
    module: Module,
    path: ItemPathBuf,
}

impl<'a> ModuleBuilder<'a> {
    fn new(items: &'a mut ItemTable, path: ItemPathBuf) -> Self {
        let (symbol, item) = match path.as_symbol_slice() {
            [symbol] => {
                let item = items.insert_if_not_exists(&path).unwrap();
                (*symbol, item)
            }
            [.., symbol] => {
                let item = items.get_item(&path).unwrap();
                (*symbol, item)
            }
            _ => panic!("empty module path"),
        };

        let module = Module {
            symbol,
            children: FxHashMap::default(),
        };

        Self {
            items,
            item,
            module,
            path: path.into(),
        }
    }

    pub fn add_item(&mut self, symbol: Symbol) {
        let child_item = self
            .items
            .insert_if_not_exists(&self.path.append(symbol))
            .expect("TODO: return duplicate error");

        self.module.children.insert(symbol, child_item);
    }
}

impl Drop for ModuleBuilder<'_> {
    fn drop(&mut self) {
        self.items.modules.insert(
            self.item,
            std::mem::replace(
                &mut self.module,
                Module {
                    symbol: Symbol::dummy(),
                    children: FxHashMap::default(),
                },
            ),
        );
    }
}
