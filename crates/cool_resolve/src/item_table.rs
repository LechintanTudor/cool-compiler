use cool_arena::{SliceArena, SliceHandle};
use cool_lexer::symbols::Symbol;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;

#[derive(Clone, Debug)]
pub struct Module {
    pub symbol: Symbol,
    pub children: FxHashMap<Symbol, ItemId>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ItemId(SliceHandle<Symbol>);

#[derive(Default, Debug)]
pub struct ItemTable {
    paths: SliceArena<Symbol>,
    modules: FxHashMap<ItemId, Module>,
}

impl ItemTable {
    #[inline]
    pub fn build_module(&mut self, module_path: &[Symbol]) -> ModuleBuilder {
        ModuleBuilder::new(self, module_path)
    }

    #[inline]
    pub fn get(&self, item: ItemId) -> &[Symbol] {
        self.paths.get(item.0)
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &[Symbol]> {
        self.paths.iter()
    }

    #[inline]
    fn insert_if_not_exists(&mut self, path: &[Symbol]) -> ItemId {
        let handle = self.paths.insert_if_not_exists(path).unwrap();
        ItemId(handle)
    }
}

#[derive(Debug)]
pub struct ModuleBuilder<'a> {
    items: &'a mut ItemTable,
    module_id: ItemId,
    module: Module,
    path: SmallVec<[Symbol; 4]>,
}

impl<'a> ModuleBuilder<'a> {
    fn new(items: &'a mut ItemTable, path: &[Symbol]) -> Self {
        let module_symbol = *path.last().expect("empty path");
        let module_id = items.insert_if_not_exists(path);
        let module = Module {
            symbol: module_symbol,
            children: FxHashMap::default(),
        };

        Self {
            items,
            module_id,
            module,
            path: SmallVec::from_slice(path),
        }
    }

    pub fn add_item(&mut self, symbol: Symbol) {
        self.path.push(symbol);
        let item_id = self.items.insert_if_not_exists(&self.path);
        self.path.pop();

        self.module.children.insert(symbol, item_id);
    }
}

impl Drop for ModuleBuilder<'_> {
    fn drop(&mut self) {
        self.items.modules.insert(
            self.module_id,
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
