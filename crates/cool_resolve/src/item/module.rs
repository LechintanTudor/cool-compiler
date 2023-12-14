use crate::{CrateId, Item, ItemId, ResolveContext, ResolveError, ResolveResult};
use cool_collections::ahash::AHashMap;
use cool_collections::{define_index_newtype, SmallVec};
use cool_lexer::Symbol;

define_index_newtype!(ModuleId);

impl ModuleId {
    pub const BUILTINS: Self = Self(0);
}

#[derive(Debug)]
pub struct Module {
    pub crate_id: CrateId,
    pub path: SmallVec<Symbol, 4>,
    pub items: AHashMap<Symbol, ModuleItem>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ModuleItem {
    pub is_exported: bool,
    pub item: Item,
}

impl ResolveContext {
    pub fn add_module(
        &mut self,
        parent_id: ModuleId,
        is_exported: bool,
        symbol: Symbol,
    ) -> ResolveResult<ItemId> {
        if self.modules[parent_id].items.contains_key(&symbol) {
            return Err(ResolveError::SymbolAlreadyExists { symbol });
        }

        let path = {
            let mut path: SmallVec<Symbol, 4> = SmallVec::new();
            path.extend_from_slice(&self.modules[parent_id].path);
            path.push(symbol);
            path
        };

        let module_id = self.modules.push(Module {
            crate_id: self.modules[parent_id].crate_id,
            path,
            items: AHashMap::default(),
        });

        let parent_module = &mut self.modules[parent_id];

        parent_module.items.insert(
            symbol,
            ModuleItem {
                is_exported,
                item: module_id.into(),
            },
        );

        let parent_crate = &mut self.crates[parent_module.crate_id];
        let module_path = self.modules[module_id].path.as_slice();

        let item_id = parent_crate.paths.insert_slice(module_path);
        let actual_item_id = parent_crate.items.push(module_id.into());
        debug_assert_eq!(item_id, actual_item_id);

        Ok(item_id)
    }

    pub fn add_import(
        &mut self,
        module_id: ModuleId,
        is_exported: bool,
        symbol: Symbol,
        item: Item,
    ) -> ResolveResult<ItemId> {
        let parent = &mut self.modules[module_id];

        if parent.items.contains_key(&symbol) {
            return Err(ResolveError::SymbolAlreadyExists { symbol });
        }

        parent
            .items
            .insert(symbol, ModuleItem { is_exported, item });

        let path = {
            let mut path: SmallVec<Symbol, 8> = SmallVec::new();
            path.extend_from_slice(&parent.path);
            path.push(symbol);
            path
        };

        let crate_id = self.modules[module_id].crate_id;
        let parent_crate = &mut self.crates[crate_id];

        let item_id = parent_crate.paths.insert_slice(&path);
        let actual_item_id = parent_crate.items.push(item);
        debug_assert_eq!(item_id, actual_item_id);

        Ok(item_id)
    }
}
