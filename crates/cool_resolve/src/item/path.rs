use crate::{Item, ItemId, ModuleId, ResolveContext, ResolveError, ResolveResult};
use cool_lexer::{sym, Symbol};

impl ResolveContext {
    pub fn resolve_path(&self, module_id: ModuleId, path: &[Symbol]) -> ResolveResult<ItemId> {
        let module = &self.modules[module_id];

        let (mut item_id, remaining_path) = if path[0] == sym::kw_crate {
            (self.get_crate_item_id(module.item_id), path.pop_front())
        } else if path[0] == sym::kw_super {
            (module.item_id, path)
        } else if path[0] == sym::kw_self {
            (module.item_id, path.pop_front())
        } else if let Some(item) = module.items.get(&path[0]) {
            (item.item_id, path.pop_front())
        } else if let Some(item_id) = self.get_dep_item_id(module.item_id, path[0]) {
            (item_id, path.pop_front())
        } else {
            return Err(ResolveError::ItemNotFound { path: path.into() });
        };

        for symbol in remaining_path {
            item_id = if *symbol == sym::kw_super {
                let (crate_id, crate_item_id) = self.items[item_id];
                let current_crate = &self.crates[crate_id];
                let item_path = &current_crate.paths[crate_item_id];

                if item_path.is_empty() {
                    return Err(ResolveError::ImportIsTooSuper);
                }

                current_crate
                    .paths
                    .get_index(item_path.pop_back())
                    .map(|crate_item_id| current_crate.items[crate_item_id])
                    .unwrap()
            } else {
                let Item::Module(current_module_id) = self.item_defs[item_id] else {
                    return Err(ResolveError::ItemNotFound { path: path.into() });
                };

                let Some(item) = self.modules[current_module_id].items.get(symbol) else {
                    return Err(ResolveError::ItemNotFound { path: path.into() });
                };

                if !item.is_exported && !self.is_child_item(item_id, module.item_id) {
                    return Err(ResolveError::ItemNotAccessible {
                        item_id: item.item_id,
                    });
                }

                item.item_id
            };
        }

        Ok(item_id)
    }

    #[must_use]
    fn get_crate_item_id(&self, item_id: ItemId) -> ItemId {
        let (crate_id, _) = self.items[item_id];
        self.modules[crate_id.as_module_id()].item_id
    }

    #[must_use]
    fn get_dep_item_id(&self, item_id: ItemId, symbol: Symbol) -> Option<ItemId> {
        let (crate_id, _) = self.items[item_id];
        let dep_id = *self.crates[crate_id].deps.get(&symbol)?;
        Some(self.modules[dep_id.as_module_id()].item_id)
    }

    #[must_use]
    fn is_child_item(&self, parent_item_id: ItemId, child_item_id: ItemId) -> bool {
        let (parent_crate_id, parent_crate_item_id) = self.items[parent_item_id];
        let (child_crate_id, child_crate_item_id) = self.items[child_item_id];

        if parent_crate_id != child_crate_id {
            return false;
        }

        let parent_path = &self.crates[parent_crate_id].paths[parent_crate_item_id];
        let child_path = &self.crates[child_crate_id].paths[child_crate_item_id];
        child_path.starts_with(parent_path)
    }
}

trait SymbolPath {
    #[must_use]
    fn pop_front(&self) -> &Self;

    #[must_use]
    fn pop_back(&self) -> &Self;
}

impl SymbolPath for [Symbol] {
    fn pop_front(&self) -> &Self {
        match self {
            [] => &[],
            [_, tail @ ..] => tail,
        }
    }

    fn pop_back(&self) -> &Self {
        match self {
            [] => &[],
            [head @ .., _] => head,
        }
    }
}
