use crate::item::{resolver, ItemError, ItemErrorKind, ItemPath, ItemPathBuf};
use cool_arena::{SliceArena, SliceHandle};
use cool_lexer::symbols::Symbol;
use rustc_hash::FxHashMap;
use std::collections::hash_map::Entry;
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
    pub fn insert_root_module(&mut self, symbol: Symbol) -> Result<ItemId, ItemError> {
        let path = ItemPathBuf::from(symbol);
        let module_id = self.create_item(path.as_path())?;

        self.modules.insert(
            module_id,
            Module {
                path,
                items: Default::default(),
            },
        );

        Ok(module_id)
    }

    pub fn insert_module(
        &mut self,
        parent_module_id: ItemId,
        is_exported: bool,
        symbol: Symbol,
    ) -> Result<ItemId, ItemError> {
        let (module_id, path) = self.insert_item_full(parent_module_id, is_exported, symbol)?;

        self.modules.insert(
            module_id,
            Module {
                path,
                items: Default::default(),
            },
        );

        Ok(module_id)
    }

    pub fn insert_item(
        &mut self,
        parent_module_id: ItemId,
        is_exported: bool,
        symbol: Symbol,
    ) -> Result<ItemId, ItemError> {
        self.insert_item_full(parent_module_id, is_exported, symbol)
            .map(|(id, _)| id)
    }

    fn insert_item_full(
        &mut self,
        parent_module_id: ItemId,
        is_exported: bool,
        symbol: Symbol,
    ) -> Result<(ItemId, ItemPathBuf), ItemError> {
        let Some(parent_module) = self.modules.get_mut(&parent_module_id) else {
            return Err(ItemError {
                kind: ItemErrorKind::SymbolNotFound,
                module_path: ItemPathBuf::default(),
                symbol_path: ItemPathBuf::from(symbol),
            });
        };

        let path = parent_module.path.append(symbol);
        let item_id = match parent_module.items.entry(symbol) {
            Entry::Vacant(entry) => {
                let item_id = match self.paths.insert_if_not_exists(path.as_symbol_slice()) {
                    Some(handle) => ItemId(handle),
                    None => {
                        return Err(ItemError {
                            kind: ItemErrorKind::SymbolAlreadyDefined,
                            module_path: path.pop().clone(),
                            symbol_path: path,
                        })
                    }
                };

                entry.insert(ModuleItem {
                    is_exported,
                    item_id,
                });
                item_id
            }
            Entry::Occupied(_) => {
                return Err(ItemError {
                    kind: ItemErrorKind::SymbolAlreadyDefined,
                    module_path: path.pop().clone(),
                    symbol_path: path,
                })
            }
        };

        Ok((item_id, path))
    }

    pub fn insert_use_decl(
        &mut self,
        module_id: ItemId,
        is_exported: bool,
        import_path: ItemPath,
    ) -> Result<bool, ItemError> {
        let Some(resolved_item_id) = resolver::resolve_import(self, module_id, import_path)? else {
            return Ok(false);
        };

        let symbol = import_path.last();
        let module = self.get_module_by_id_mut(module_id).unwrap().unwrap();

        if module.items.contains_key(&symbol) {
            return Err(ItemError {
                kind: ItemErrorKind::SymbolAlreadyDefined,
                module_path: module.path.clone(),
                symbol_path: import_path.to_path_buf(),
            });
        }

        module.items.insert(
            symbol,
            ModuleItem {
                is_exported,
                item_id: resolved_item_id,
            },
        );

        Ok(true)
    }

    fn create_item(&mut self, path: ItemPath) -> Result<ItemId, ItemError> {
        match self.paths.insert_if_not_exists(path.as_symbol_slice()) {
            Some(handle) => Ok(ItemId(handle)),
            None => Err(ItemError {
                kind: ItemErrorKind::SymbolAlreadyDefined,
                module_path: path.pop().to_path_buf(),
                symbol_path: path.to_path_buf(),
            }),
        }
    }

    pub fn print_final(&self) {
        for (item, module) in self.modules.iter() {
            let module_path = self.get_path_by_id(*item).unwrap();
            println!("[MODULE {}]", module_path);

            for (symbol, item) in module.items.iter() {
                let item_path = self.get_path_by_id(item.item_id).unwrap();
                println!("{}: {}", symbol, item_path);
            }

            println!()
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

    pub fn get_module_by_id_mut(
        &mut self,
        module_id: ItemId,
    ) -> Result<Option<&mut Module>, ItemErrorKind> {
        match self.modules.get_mut(&module_id) {
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
