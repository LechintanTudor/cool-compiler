use crate::{ItemId, ItemKind, ModuleId, ResolveContext, ResolveError, ResolveResult, Scope};
use cool_lexer::{sym, Symbol};

impl ResolveContext<'_> {
    pub fn get_symbol<S>(&self, scope: S, symbol: Symbol) -> ResolveResult<ItemKind>
    where
        S: Into<Scope>,
    {
        let mut scope = scope.into();

        loop {
            match scope {
                Scope::Frame(frame_id) => {
                    let frame = &self[frame_id];

                    match frame.get(symbol) {
                        Some(binding_id) => return Ok(binding_id.into()),
                        None => scope = frame.parent,
                    }
                }
                Scope::Module(module_id) => {
                    return self
                        .resolve_path(module_id, &[symbol])
                        .map(|item_id| self[item_id]);
                }
            }
        }
    }

    pub fn resolve_path(&self, module_id: ModuleId, path: &[Symbol]) -> ResolveResult<ItemId> {
        let module = &self.modules[module_id];

        let (mut current_path, remaining_path) = if path[0] == sym::kw_crate {
            (&self.get_module_path(module_id)[..1], &path[1..])
        } else if path[0] == sym::kw_super {
            (parent_path(self.get_module_path(module_id)), &path[1..])
        } else if path[0] == sym::kw_self {
            (self.get_module_path(module_id), &path[1..])
        } else if module.elems.contains_key(&path[0]) {
            (self.get_module_path(module_id), path)
        } else {
            (&path[..1], &path[1..])
        };

        for &symbol in remaining_path {
            if symbol == sym::kw_super {
                if current_path.is_empty() {
                    return Err(ResolveError::ImportIsTooSuper);
                }

                current_path = &current_path[..(current_path.len() - 1)];
                continue;
            }

            let module_id = self.get_module_by_path(current_path)?;

            let item = self.modules[module_id].elems.get(&symbol).ok_or_else(|| {
                ResolveError::ItemNotFound {
                    path: current_path.into(),
                }
            })?;

            if !item.is_exported && !self.is_child_item(module.item_id, item.item_id) {
                return Err(ResolveError::ItemNotAccessible {
                    item_id: item.item_id,
                });
            }

            current_path = self.get_path(item.item_id);
        }

        self.get_item_by_path(current_path)
    }

    fn get_item_by_path(&self, path: &[Symbol]) -> ResolveResult<ItemId> {
        self.paths
            .get_index(path)
            .ok_or_else(|| ResolveError::ItemNotFound { path: path.into() })
    }

    fn get_module_by_path(&self, path: &[Symbol]) -> ResolveResult<ModuleId> {
        let item_id = self.get_item_by_path(path)?;

        self.items[&item_id]
            .try_into()
            .map_err(|_| ResolveError::ItemNotModule { item_id })
    }

    #[must_use]
    fn is_child_item(&self, parent_id: ItemId, item_id: ItemId) -> bool {
        let parent_path = self.get_path(parent_id);
        let child_path = self.get_path(item_id);
        child_path.starts_with(parent_path)
    }
}

#[inline]
#[must_use]
fn parent_path(path: &[Symbol]) -> &[Symbol] {
    &path[..(path.len() - 1)]
}
