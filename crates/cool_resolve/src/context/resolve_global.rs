use crate::{
    Binding, ItemId, ItemKind, ItemPath, ItemPathBuf, Module, ModuleElem, Mutability,
    ResolveContext, ResolveError, ResolveErrorKind, ResolveResult, Scope,
};
use cool_collections::id_newtype;
use cool_lexer::{sym, Symbol};
use std::ops::{self, Deref};

id_newtype!(ModuleId);

impl ModuleId {
    #[inline]
    pub const fn for_builtins() -> Self {
        unsafe { Self::new_unchecked(1) }
    }
}

impl ResolveContext {
    pub(crate) fn insert_item(&mut self, path: &[Symbol]) -> ResolveResult<ItemId> {
        if self.paths.contains(path) {
            return Err(ResolveError {
                symbol: path.last().copied().unwrap_or(sym::EMPTY),
                kind: ResolveErrorKind::SymbolAlreadyDefined,
            });
        }

        Ok(ItemId::from(self.paths.insert_slice(path)))
    }

    pub fn insert_root_module(&mut self, symbol: Symbol) -> ResolveResult<ModuleId> {
        let item_id = self.insert_item(&[symbol])?;
        let module_id = self.modules.push(Module::new(item_id));
        self.items.insert(item_id, ItemKind::Module(module_id));

        Ok(module_id)
    }

    pub fn insert_module(
        &mut self,
        parent: ModuleId,
        is_exported: bool,
        symbol: Symbol,
    ) -> ResolveResult<ModuleId> {
        let item_id = {
            let parent_module = &self.modules[parent];
            let path = ItemPathBuf::from_base_and_symbol(&parent_module.item_id, symbol);
            self.insert_item(path.as_symbol_slice())?
        };

        let module_id = self.modules.push(Module::new(item_id));
        self.items.insert(item_id, ItemKind::Module(module_id));

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
        let item_path = parent_module.child_path(symbol);

        let item_id = self
            .paths
            .insert_slice_if_not_exists(item_path.as_symbol_slice())
            .map(ItemId::from)
            .ok_or(ResolveError {
                symbol,
                kind: ResolveErrorKind::SymbolAlreadyDefined,
            })?;

        let binding_id = self.bindings.push(Binding {
            symbol,
            mutability,
            ty_id: self.tys.consts().infer,
        });

        self.items.insert(item_id, ItemKind::Binding(binding_id));

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
        let first_symbol = path.first();
        let mut symbol_iter = path.as_symbol_slice().iter();

        let mut resolved_path: ItemPathBuf = match first_symbol {
            sym::KW_CRATE => {
                let _ = symbol_iter.next();
                module.path().first().into()
            }
            sym::KW_SUPER => {
                let _ = symbol_iter.next();
                module
                    .path()
                    .try_pop()
                    .ok_or(ResolveError {
                        symbol: path.last(),
                        kind: ResolveErrorKind::TooManySuperKeywords,
                    })?
                    .to_path_buf()
            }
            sym::KW_SELF => {
                let _ = symbol_iter.next();
                module.path().to_path_buf()
            }
            symbol => {
                if module.elems.contains_key(&symbol) {
                    module.path().to_path_buf()
                } else if self.paths.contains(&[symbol]) {
                    // Check other crates
                    ItemPathBuf::from(symbol)
                } else if self.paths.contains(&[sym::EMPTY, symbol]) {
                    // Check builtins
                    ItemPathBuf::from(sym::EMPTY)
                } else {
                    return Err(ResolveError {
                        symbol,
                        kind: ResolveErrorKind::SymbolNotFound,
                    });
                }
            }
        };

        for &symbol in symbol_iter {
            let current_module = self.get_module_by_path(&resolved_path)?;

            let Some(current_item) = current_module.elems.get(&symbol) else {
                return Err(ResolveError {
                    symbol,
                    kind: ResolveErrorKind::SymbolNotFound,
                });
            };

            if !current_item.is_exported && !module.item_id.is_child_of(current_module.item_id) {
                return Err(ResolveError {
                    symbol,
                    kind: ResolveErrorKind::SymbolNotPublic,
                });
            }

            resolved_path = current_item.item_id.deref().into();
        }

        self.get_item_id_by_path(&resolved_path)
            .ok_or(ResolveError {
                symbol: resolved_path.last(),
                kind: ResolveErrorKind::SymbolNotFound,
            })
    }

    #[inline]
    fn get_item_id_by_path<'a, P>(&self, path: P) -> Option<ItemId>
    where
        P: Into<ItemPath<'a>>,
    {
        self.paths
            .get(path.into().as_symbol_slice())
            .map(ItemId::from)
    }

    fn get_module_by_path<'a, P>(&self, path: P) -> ResolveResult<&Module>
    where
        P: Into<ItemPath<'a>>,
    {
        let path: ItemPath = path.into();

        let item_id = self
            .paths
            .get(path.as_symbol_slice())
            .map(ItemId::from)
            .ok_or(ResolveError {
                symbol: path.last(),
                kind: ResolveErrorKind::SymbolNotFound,
            })?;

        let module_id = self.items[&item_id].as_module_id().ok_or(ResolveError {
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
        &self.items[&item_id]
    }
}

impl ops::Index<ModuleId> for ResolveContext {
    type Output = Module;

    #[inline]
    fn index(&self, module_id: ModuleId) -> &Self::Output {
        &self.modules[module_id]
    }
}
