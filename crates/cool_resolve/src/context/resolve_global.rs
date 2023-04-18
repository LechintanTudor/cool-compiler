use crate::{
    tys, Binding, ItemKind, ItemPath, ItemPathBuf, Module, ModuleElem, Mutability, ResolveContext,
    ResolveError, ResolveErrorKind, ResolveResult, ScopeId,
};
use cool_collections::id_newtype;
use cool_lexer::symbols::{sym, Symbol};
use std::ops;

id_newtype!(ItemId);
id_newtype!(ModuleId);

impl ModuleId {
    #[inline]
    pub fn for_builtins() -> Self {
        Self::new_unwrap(1)
    }
}

impl ResolveContext {
    pub fn insert_root_module(&mut self, symbol: Symbol) -> ResolveResult<ModuleId> {
        let item_id = self
            .paths
            .insert_if_not_exists(&[symbol])
            .ok_or(ResolveError::already_defined(symbol))?;

        let module_id = self.modules.push(Module::from_path(symbol));

        self.items
            .push_checked(item_id, ItemKind::Module(module_id));

        Ok(module_id)
    }

    pub fn insert_module(
        &mut self,
        parent_id: ModuleId,
        is_exported: bool,
        symbol: Symbol,
    ) -> ResolveResult<ModuleId> {
        let module_path = {
            let parent_module = &self.modules[parent_id];
            parent_module.path.append(symbol)
        };

        let item_id = self
            .paths
            .insert_if_not_exists(module_path.as_symbol_slice())
            .ok_or(ResolveError::already_defined(symbol))?;

        let module_id = self.modules.push(Module::from_path(module_path));

        self.items
            .push_checked(item_id, ItemKind::Module(module_id));

        let parent_module = &mut self.modules[parent_id];
        parent_module.elems.insert(
            symbol,
            ModuleElem {
                is_exported,
                item_id,
            },
        );

        Ok(module_id)
    }

    pub fn insert_global_binding(
        &mut self,
        parent_id: ModuleId,
        is_exported: bool,
        mutability: Mutability,
        symbol: Symbol,
    ) -> ResolveResult<ItemId> {
        let parent_module = &mut self.modules[parent_id];
        let item_path = parent_module.path.append(symbol);

        let item_id = self
            .paths
            .insert_if_not_exists(item_path.as_symbol_slice())
            .ok_or(ResolveError::already_defined(symbol))?;

        let binding_id = self.bindings.push(Binding {
            mutability,
            ty_id: tys::INFERRED,
        });

        self.items
            .push_checked(item_id, ItemKind::Binding(binding_id));

        parent_module.elems.insert(
            symbol,
            ModuleElem {
                is_exported,
                item_id,
            },
        );

        Ok(item_id)
    }

    pub fn insert_use<'a, P>(
        &mut self,
        parent_id: ModuleId,
        is_exported: bool,
        path: P,
        alias: Option<Symbol>,
    ) -> ResolveResult<ItemId>
    where
        P: Into<ItemPath<'a>>,
    {
        let path: ItemPath = path.into();
        let item_id = self.resolve_global(parent_id.into(), path)?;
        let module = &mut self.modules[parent_id];
        let symbol = alias.unwrap_or(path.last());

        if module.elems.contains_key(&symbol) {
            return Err(ResolveError::already_defined(symbol));
        }

        module.elems.insert(
            symbol,
            ModuleElem {
                is_exported,
                item_id,
            },
        );

        Ok(item_id)
    }

    pub fn resolve_parent_module(&self, mut scope_id: ScopeId) -> ModuleId {
        loop {
            match scope_id {
                ScopeId::Frame(frame_id) => {
                    scope_id = self.frames[frame_id].parent_id;
                }
                ScopeId::Module(module_id) => break module_id,
            }
        }
    }

    pub fn resolve_global<'a, P>(&self, scope_id: ScopeId, path: P) -> ResolveResult<ItemId>
    where
        P: Into<ItemPath<'a>>,
    {
        let module_id = self.resolve_parent_module(scope_id);
        let module = &self.modules[module_id];

        let path: ItemPath = path.into();
        let mut path_iter = path.as_symbol_slice().iter();

        let mut resolved_path: ItemPathBuf = match *path_iter.next().unwrap() {
            sym::KW_CRATE => module.path.first().into(),
            sym::KW_SUPER => {
                module
                    .path
                    .try_pop()
                    .map(|path| path.clone())
                    .ok_or(ResolveError {
                        symbol: path.last(),
                        kind: ResolveErrorKind::TooManySuperKeywords,
                    })?
            }
            sym::KW_SELF => module.path.clone(),
            symbol => {
                if module.elems.contains_key(&symbol) {
                    // Check local module
                    module.path.append(symbol)
                } else if self.paths.contains(&[symbol]) {
                    // Check other crates
                    ItemPathBuf::from(symbol)
                } else if self.paths.contains(&[sym::EMPTY, symbol]) {
                    // Check builtins
                    ItemPathBuf::from([sym::EMPTY, symbol].as_slice())
                } else {
                    return Err(ResolveError::not_found(symbol));
                }
            }
        };

        for &symbol in path_iter {
            let current_module = self.get_module_by_path(&resolved_path)?;

            let current_item = current_module
                .elems
                .get(&symbol)
                .ok_or(ResolveError::not_found(symbol))?;

            if !current_item.is_exported && !module.path.starts_with(&current_module.path) {
                return Err(ResolveError::private(symbol));
            }

            resolved_path = resolved_path.append(symbol);
        }

        self.get_item_id_by_path(&resolved_path)
            .ok_or(ResolveError::not_found(resolved_path.last()))
    }

    #[inline]
    fn get_item_id_by_path<'a, P>(&self, path: P) -> Option<ItemId>
    where
        P: Into<ItemPath<'a>>,
    {
        self.paths.get_id(path.into().as_symbol_slice())
    }

    fn get_module_by_path<'a, P>(&self, path: P) -> ResolveResult<&Module>
    where
        P: Into<ItemPath<'a>>,
    {
        let path: ItemPath = path.into();

        let item_id = self
            .paths
            .get_id(path.as_symbol_slice())
            .ok_or(ResolveError::not_found(path.last()))?;

        let module_id = self.items[item_id]
            .as_module_id()
            .ok_or(ResolveError::not_module(path.last()))?;

        Ok(&self.modules[module_id])
    }
}

impl ops::Index<ItemId> for ResolveContext {
    type Output = ItemKind;

    #[inline]
    fn index(&self, item_id: ItemId) -> &Self::Output {
        &self.items[item_id]
    }
}

impl ops::Index<ModuleId> for ResolveContext {
    type Output = Module;

    #[inline]
    fn index(&self, module_id: ModuleId) -> &Self::Output {
        &self.modules[module_id]
    }
}
