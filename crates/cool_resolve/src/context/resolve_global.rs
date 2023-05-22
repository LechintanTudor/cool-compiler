use crate::{
    tys, Binding, ItemKind, ItemPath, ItemPathBuf, Module, ModuleElem, Mutability, ResolveContext,
    ResolveError, ResolveErrorKind, ResolveResult, Scope,
};
use cool_collections::id_newtype;
use cool_lexer::symbols::{sym, Symbol};
use std::ops;

id_newtype!(ItemId);
id_newtype!(ModuleId);

impl ModuleId {
    #[inline]
    pub const fn for_builtins() -> Self {
        unsafe { Self::new_unchecked(1) }
    }
}

impl ResolveContext {
    pub fn insert_root_module(&mut self, symbol: Symbol) -> ResolveResult<ModuleId> {
        let item_id = self
            .paths
            .insert_if_not_exists(&[symbol])
            .ok_or(ResolveError {
                symbol,
                kind: ResolveErrorKind::SymbolAlreadyDefined,
            })?;

        let module_id = self.modules.push(Module::from_path(symbol));

        self.items
            .push_checked(item_id, ItemKind::Module(module_id));

        Ok(module_id)
    }

    pub fn insert_module(
        &mut self,
        parent: ModuleId,
        is_exported: bool,
        symbol: Symbol,
    ) -> ResolveResult<ModuleId> {
        let module_path = {
            let parent_module = &self.modules[parent];
            parent_module.path.append(symbol)
        };

        let item_id = self
            .paths
            .insert_if_not_exists(module_path.as_symbol_slice())
            .ok_or(ResolveError {
                symbol,
                kind: ResolveErrorKind::SymbolAlreadyDefined,
            })?;

        let module_id = self.modules.push(Module::from_path(module_path));

        self.items
            .push_checked(item_id, ItemKind::Module(module_id));

        let parent_module = &mut self.modules[parent];
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
        parent: ModuleId,
        is_exported: bool,
        mutability: Mutability,
        symbol: Symbol,
    ) -> ResolveResult<ItemId> {
        let parent_module = &mut self.modules[parent];
        let item_path = parent_module.path.append(symbol);

        let item_id = self
            .paths
            .insert_if_not_exists(item_path.as_symbol_slice())
            .ok_or(ResolveError {
                symbol,
                kind: ResolveErrorKind::SymbolAlreadyDefined,
            })?;

        let binding_id = self.bindings.push(Binding {
            symbol,
            mutability,
            ty_id: tys::INFER,
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
        parent: ModuleId,
        is_exported: bool,
        path: P,
        alias: Option<Symbol>,
    ) -> ResolveResult<ItemId>
    where
        P: Into<ItemPath<'a>>,
    {
        let path: ItemPath = path.into();
        let item_id = self.resolve_global(parent.into(), path)?;
        let module = &mut self.modules[parent];
        let symbol = alias.unwrap_or(path.last());

        if module.elems.contains_key(&symbol) {
            return Err(ResolveError {
                symbol,
                kind: ResolveErrorKind::SymbolAlreadyDefined,
            });
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

    pub fn resolve_parent_module(&self, mut scope: Scope) -> ModuleId {
        loop {
            match scope {
                Scope::Frame(frame_id) => {
                    scope = self.frames[frame_id].parent;
                }
                Scope::Module(module_id) => break module_id,
            }
        }
    }

    pub fn resolve_global<'a, P>(&self, scope: Scope, path: P) -> ResolveResult<ItemId>
    where
        P: Into<ItemPath<'a>>,
    {
        let module_id = self.resolve_parent_module(scope);
        let module = &self.modules[module_id];

        let path: ItemPath = path.into();
        let mut path_iter = path.as_symbol_slice().iter();

        let mut resolved_path: ItemPathBuf = match *path_iter.next().unwrap() {
            sym::KW_CRATE => module.path.first().into(),
            sym::KW_SUPER => {
                module.path.try_pop().ok_or(ResolveError {
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
                    return Err(ResolveError {
                        symbol,
                        kind: ResolveErrorKind::SymbolNotFound,
                    });
                }
            }
        };

        for &symbol in path_iter {
            let current_module = self.get_module_by_path(&resolved_path)?;

            let current_item = current_module.elems.get(&symbol).ok_or(ResolveError {
                symbol,
                kind: ResolveErrorKind::SymbolNotFound,
            })?;

            if !current_item.is_exported && !module.path.starts_with(&current_module.path) {
                return Err(ResolveError {
                    symbol,
                    kind: ResolveErrorKind::SymbolNotPublic,
                });
            }

            resolved_path = resolved_path.append(symbol);
        }

        self.get_item_id_by_path(&resolved_path)
            .ok_or(ResolveError {
                symbol: resolved_path.last(),
                kind: ResolveErrorKind::SymbolNotFound,
            })
    }

    #[inline]
    pub fn get_path_by_item_id(&self, item_id: ItemId) -> ItemPath {
        self.paths[item_id].into()
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
            .ok_or(ResolveError {
                symbol: path.last(),
                kind: ResolveErrorKind::SymbolNotFound,
            })?;

        let module_id = self.items[item_id].as_module_id().ok_or(ResolveError {
            symbol: path.last(),
            kind: ResolveErrorKind::SymbolNotModule,
        })?;

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
