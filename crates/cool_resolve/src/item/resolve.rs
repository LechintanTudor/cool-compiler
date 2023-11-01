use crate::{ItemError, ItemId, ItemResult, ModuleId, ResolveContext};
use cool_lexer::{sym, Symbol};

impl ResolveContext<'_> {
    pub fn resolve_path(&self, module_id: ModuleId, path: &[Symbol]) -> ItemResult<ItemId> {
        let module = &self.modules[module_id];

        let mut current_path: &[Symbol] = if path[0] == sym::kw_crate {
            &self.get_module_path(module_id)[..1]
        } else if path[0] == sym::kw_super {
            let path = self.get_module_path(module_id);
            &path[..(path.len() - 1)]
        } else if path.len() >= 2 && self[module_id].elems.contains_key(&path[1]) {
            self.get_module_path(module_id)
        } else {
            &path[..1]
        };

        for &symbol in &path[1..] {
            if symbol == sym::kw_super {
                if current_path.is_empty() {
                    return Err(ItemError::TooManySuper);
                }

                current_path = &current_path[..(current_path.len() - 1)];
                continue;
            }

            let module_id = self.get_module_by_path(current_path)?;

            let item = self.modules[module_id].elems.get(&symbol).ok_or_else(|| {
                ItemError::NotFound {
                    path: current_path.into(),
                }
            })?;

            if !item.is_exported && !self.is_child_item(module.item_id, item.item_id) {
                return Err(ItemError::NotAccessible {
                    item_id: item.item_id,
                });
            }

            current_path = self.get_path(item.item_id);
        }

        self.get_item_by_path(current_path)
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
