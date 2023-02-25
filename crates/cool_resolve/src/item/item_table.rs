use crate::item::{ItemPath, ItemPathBuf};
use cool_arena::{SliceArena, SliceHandle};
use cool_lexer::symbols::Symbol;
use rustc_hash::FxHashMap;

#[derive(Clone, Debug)]
pub struct Module {
    pub symbol: Symbol,
    pub children: FxHashMap<Symbol, ModuleItem>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ModuleItem {
    pub is_exported: bool,
    pub id: ItemId,
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
    pub fn build_module(&mut self, path: ItemPathBuf) -> ModuleBuilder {
        ModuleBuilder::new(self, path)
    }

    #[inline]
    fn insert_if_not_exists<'a, P>(&mut self, path: P) -> Option<ItemId>
    where
        P: Into<ItemPath<'a>>,
    {
        self.paths
            .insert_if_not_exists(path.into().as_symbol_slice())
            .map(ItemId)
    }

    // Only supports parents importing from children for now
    pub fn insert_use_decl(
        &mut self,
        module_id: ItemId,
        is_exported: bool,
        item_path: ItemPath,
    ) -> bool {
        let item = {
            let module = self.modules.get(&module_id).expect("invalid module id");
            let (first_symbol, other_symbols) = item_path.as_symbol_slice().split_first().unwrap();

            let child_module_id = match module.children.get(first_symbol) {
                Some(child_module) => child_module.id,
                None => return false,
            };

            let mut child_module = self.modules.get(&child_module_id).unwrap();
            let (last_symbol, other_symbols) = other_symbols.split_last().unwrap();

            for symbol in other_symbols {
                let Some(item) = child_module.children.get(symbol) else {
                return false;
            };

                if !item.is_exported {
                    panic!("tried to use private item");
                }

                let Some(next_child_module) = self.modules.get(&item.id) else {
                return false;
            };

                child_module = next_child_module;
            }

            let Some(item) = child_module.children.get(last_symbol) else {
            return false;
        };

            if !item.is_exported {
                panic!("tried to use private item");
            }

            item.id
        };

        let module = self.modules.get_mut(&module_id).expect("invalid module id");
        let symbol = item_path.as_symbol_slice().last().unwrap();

        if module.children.contains_key(symbol) {
            panic!("duplicate items");
        }

        module.children.insert(
            *symbol,
            ModuleItem {
                is_exported,
                id: item,
            },
        );
        true
    }

    pub fn print_final(&self) {
        for (item, module) in self.modules.iter() {
            let path = self.get(*item);
            println!("[MODULE {path}]");

            for symbol in module.children.keys() {
                println!("- {symbol}");
            }

            println!();
        }
    }

    #[inline]
    pub fn get_item<'a, P>(&self, path: P) -> Option<ItemId>
    where
        P: Into<ItemPath<'a>>,
    {
        self.paths
            .get_handle(path.into().as_symbol_slice())
            .map(ItemId)
    }

    #[inline]
    pub fn get(&self, item: ItemId) -> ItemPath {
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
    id: ItemId,
    module: Module,
    path: ItemPathBuf,
}

impl<'a> ModuleBuilder<'a> {
    fn new(items: &'a mut ItemTable, path: ItemPathBuf) -> Self {
        let (symbol, id) = match path.as_symbol_slice() {
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
            id,
            module,
            path: path.into(),
        }
    }

    #[inline]
    pub fn id(&self) -> ItemId {
        self.id
    }

    pub fn add_item(&mut self, is_exported: bool, symbol: Symbol) {
        let id = self
            .items
            .insert_if_not_exists(&self.path.append(symbol))
            .expect("TODO: return duplicate error");

        self.module
            .children
            .insert(symbol, ModuleItem { is_exported, id });
    }
}

impl Drop for ModuleBuilder<'_> {
    fn drop(&mut self) {
        self.items.modules.insert(
            self.id,
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
