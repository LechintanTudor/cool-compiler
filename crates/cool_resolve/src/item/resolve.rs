use crate::{ItemError, ItemId, ItemResult, ModuleId, ResolveContext};
use cool_lexer::{sym, Symbol};
use std::slice;

impl ResolveContext<'_> {
    pub fn resolve_path(&self, module_id: ModuleId, path: &[Symbol]) -> ItemResult<ItemId> {
        let module = &self.modules[module_id];
        let (&first_symbol, other_symbols) = path.split_first().unwrap();

        let mut path: &[Symbol] = if first_symbol == sym::kw_crate {
            slice::from_ref(&self.get_module_path(module_id)[0])
        } else if self[module_id].elems.contains_key(&other_symbols[0]) {
            self.get_module_path(module_id)
        } else {
            slice::from_ref(&first_symbol)
        };

        for symbol in other_symbols {
            let module_id = self.get_module_by_path(path)?;

            let item = self.modules[module_id]
                .elems
                .get(symbol)
                .ok_or_else(|| ItemError::NotFound { path: path.into() })?;

            if !item.is_exported && !self.is_child_item(module.item_id, item.item_id) {
                return Err(ItemError::NotAccessible {
                    item_id: item.item_id,
                });
            }

            path = self.get_path(item.item_id);
        }

        self.get_item_by_path(path)
    }

    fn get_item_by_path(&self, path: &[Symbol]) -> ItemResult<ItemId> {
        self.paths
            .get_index(path)
            .ok_or_else(|| ItemError::NotFound { path: path.into() })
    }

    fn get_module_by_path(&self, path: &[Symbol]) -> ItemResult<ModuleId> {
        let item_id = self.get_item_by_path(path)?;

        self.items[&item_id]
            .try_into()
            .map_err(|_| ItemError::NotModule { item_id })
    }

    #[must_use]
    fn is_child_item(&self, parent_id: ItemId, item_id: ItemId) -> bool {
        let parent_path = self.get_path(parent_id);
        let child_path = self.get_path(item_id);
        child_path.starts_with(parent_path)
    }
}
