use crate::item::{ItemError, ItemErrorKind, ItemPath, ItemPathBuf};
use cool_arena::{SliceArena, SliceHandle};
use cool_collections::SmallVecSet;
use cool_lexer::symbols::{sym, Symbol};
use rustc_hash::FxHashMap;
use std::fmt;

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
    pub parents: SmallVecSet<ItemId, 4>,
    pub items: FxHashMap<Symbol, ModuleItem>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ModuleItem {
    pub is_exported: bool,
    pub item_id: ItemId,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ItemId(SliceHandle<Symbol>);

impl fmt::Debug for ItemId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ItemId").field(&self.0.index()).finish()
    }
}

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

    fn try_resolve_import(
        &mut self,
        module_id: ItemId,
        is_exported: bool,
        item_path: ItemPath,
    ) -> Result<bool, ItemError> {
        let module = self.modules.get(&module_id).expect("module not found");

        if item_path.starts_with_self_or_super() {
        } else if item_path.starts_with_crate() {
        } else {
        }

        todo!()
    }

    fn try_resolve_local_import(&self, module_path: ItemPath, item_path: ItemPath) {
        let mut resolved_path = module_path.to_path_buf();

        for symbol in item_path.as_symbol_slice().iter() {
            resolved_path = match *symbol {
                sym::KW_SELF => continue,
                sym::KW_SUPER => resolved_path.pop(), // TODO: Check if path is empty
                symbol => resolved_path.append(symbol),
            }
        }

        // this_crate.this_module.this_submodule
        // super.other_module -> this_crate.other_module
    }

    fn try_resolve_crate_import(&self, module_id: ItemId, is_exported: bool, item_path: ItemPath) {}

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
            let path = self.get_path_by_id_unwrap(*item);
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
    pub fn get_path_by_id_unwrap(&self, item: ItemId) -> ItemPath {
        self.paths[item.0].into()
    }

    #[inline]
    pub fn get_path_by_id(&self, item_id: ItemId) -> Option<ItemPath> {
        self.paths.get(item_id.0).map(|path| path.into())
    }

    #[inline]
    pub fn get_module_by_id(&self, module_id: ItemId) -> Result<Option<&Module>, ItemErrorKind> {
        match self.modules.get(&module_id) {
            Some(module) => Ok(Some(module)),
            None => {
                if self.paths.contains_handle(module_id.0) {
                    Err(ItemErrorKind::SymbolIsNotModule)
                } else {
                    Ok(None)
                }
            }
        }
    }

    pub fn get_module_by_path<'a, P>(&self, path: P) -> Result<Option<&Module>, ItemErrorKind>
    where
        P: Into<ItemPath<'a>>,
    {
        let Some(module_id) = self
            .paths
            .get_handle(path.into().as_symbol_slice()).map(ItemId) else {
                return Ok(None)
            };

        self.get_module_by_id(module_id)
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

        let parents = {
            let mut parents = SmallVecSet::<ItemId, 4>::default();
            let mut parent_module_path = module_path.as_path().pop();

            while !parent_module_path.is_empty() {
                let parent_module_id = items
                    .get_item(parent_module_path)
                    .expect("parent module not found");

                parents.insert(parent_module_id);
                parent_module_path = parent_module_path.pop();
            }

            parents
        };

        let module = Module {
            path: module_path,
            parents,
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
