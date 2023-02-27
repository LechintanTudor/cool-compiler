use crate::item::{ItemError, ItemErrorKind, ItemPath, ItemPathBuf};
use cool_arena::{SliceArena, SliceHandle};
use cool_lexer::symbols::Symbol;
use rustc_hash::FxHashMap;

/*
Check order
1) Local
2) Global
3) Glob imports
3) Builtins

*/

#[derive(Clone, Default, Debug)]
pub struct Module {
    pub path: ItemPathBuf,
    pub items: FxHashMap<Symbol, ModuleItem>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ModuleItem {
    pub is_exported: bool,
    pub item_id: ItemId,
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
    pub fn build_module(&mut self, path: ItemPathBuf) -> Result<ModuleBuilder, ItemError> {
        ModuleBuilder::new(self, path)
    }

    #[inline]
    fn insert_if_not_exists<'a, P>(&mut self, path: P) -> Result<ItemId, ItemError>
    where
        P: Into<ItemPath<'a>>,
    {
        let path: ItemPath = path.into();

        let item_id = self
            .paths
            .insert_if_not_exists(path.as_symbol_slice())
            .map(ItemId);

        let Some(item_id) = item_id else {
            return Err(ItemError {
                kind: ItemErrorKind::SymbolAlreadyDefined,
                module_path: path.pop().to_path_buf(),
                symbol_path: path.to_path_buf(),
            })?;
        };

        Ok(item_id)
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

            let child_module_id = match module.items.get(first_symbol) {
                Some(child_module) => child_module.item_id,
                None => return false,
            };

            let mut child_module = self.modules.get(&child_module_id).unwrap();
            let (last_symbol, other_symbols) = other_symbols.split_last().unwrap();

            for symbol in other_symbols {
                let Some(item) = child_module.items.get(symbol) else {
                return false;
            };

                if !item.is_exported {
                    panic!("tried to use private item");
                }

                let Some(next_child_module) = self.modules.get(&item.item_id) else {
                return false;
            };

                child_module = next_child_module;
            }

            let Some(item) = child_module.items.get(last_symbol) else {
            return false;
        };

            if !item.is_exported {
                panic!("tried to use private item");
            }

            item.item_id
        };

        let module = self.modules.get_mut(&module_id).expect("invalid module id");
        let symbol = item_path.as_symbol_slice().last().unwrap();

        if module.items.contains_key(symbol) {
            panic!("duplicate items");
        }

        module.items.insert(
            *symbol,
            ModuleItem {
                is_exported,
                item_id: item,
            },
        );
        true
    }

    pub fn print_final(&self) {
        for (item, module) in self.modules.iter() {
            let path = self.get(*item);
            println!("[MODULE {path}]");

            for symbol in module.items.keys() {
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
        self.paths[item.0].into()
    }

    #[inline]
    pub fn iter_paths(&self) -> impl Iterator<Item = ItemPath> {
        self.paths.iter().map(ItemPath::from)
    }
}

#[derive(Debug)]
pub struct ModuleBuilder<'a> {
    items: &'a mut ItemTable,
    item_id: ItemId,
    module: Module,
}

impl<'a> ModuleBuilder<'a> {
    fn new(items: &'a mut ItemTable, module_path: ItemPathBuf) -> Result<Self, ItemError> {
        let item_id = match module_path.len() {
            0 => panic!("empty module path"),
            1 => items.insert_if_not_exists(&module_path)?,
            _ => items.get_item(&module_path).ok_or_else(|| ItemError {
                kind: ItemErrorKind::SymbolNotFound,
                module_path: module_path.clone(),
                symbol_path: module_path.clone(),
            })?,
        };

        let module = Module {
            path: module_path,
            items: Default::default(),
        };

        Ok(Self {
            items,
            item_id,
            module,
        })
    }

    #[inline]
    pub fn item_id(&self) -> ItemId {
        self.item_id
    }

    pub fn add_item(&mut self, is_exported: bool, symbol: Symbol) -> Result<(), ItemError> {
        let item_id = self
            .items
            .insert_if_not_exists(&self.module.path.append(symbol))?;

        self.module.items.insert(
            symbol,
            ModuleItem {
                is_exported,
                item_id,
            },
        );

        Ok(())
    }
}

impl Drop for ModuleBuilder<'_> {
    fn drop(&mut self) {
        self.items
            .modules
            .insert(self.item_id, std::mem::take(&mut self.module));
    }
}
