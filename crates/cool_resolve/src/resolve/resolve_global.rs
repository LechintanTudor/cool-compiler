use crate::resolve::{
    ItemId, Module, ModuleElem, ModuleId, ResolveError, ResolveErrorKind, ResolveResult,
    ResolveTable, ScopeId, SymbolKind,
};
use crate::{ItemPath, ItemPathBuf};
use cool_lexer::symbols::{sym, Symbol};

impl ResolveTable {
    pub fn insert_root_module(&mut self, symbol: Symbol) -> ResolveResult<ModuleId> {
        let module_id = self
            .items
            .insert_if_not_exists(&[symbol])
            .map(|id| ModuleId(id.0))
            .ok_or(ResolveError::already_defined(symbol))?;

        self.modules.insert(module_id, Module::from_path(symbol));
        Ok(module_id)
    }

    pub fn insert_module(
        &mut self,
        parent_id: ModuleId,
        is_exported: bool,
        symbol: Symbol,
    ) -> ResolveResult<ModuleId> {
        let parent_module = self.modules.get_mut(&parent_id).unwrap();
        let module_path = parent_module.path.append(symbol);

        let module_id = self
            .items
            .insert_if_not_exists(module_path.as_symbol_slice())
            .map(|id| ModuleId(id.0))
            .ok_or(ResolveError::already_defined(symbol))?;

        parent_module.elems.insert(
            symbol,
            ModuleElem {
                is_exported,
                kind: SymbolKind::Item(module_id.as_item_id()),
            },
        );

        self.modules
            .insert(module_id, Module::from_path(module_path));

        Ok(module_id)
    }

    pub fn insert_item(
        &mut self,
        parent_id: ModuleId,
        is_exported: bool,
        symbol: Symbol,
    ) -> ResolveResult<ItemId> {
        let parent_module = self.modules.get_mut(&parent_id).unwrap();
        let module_path = parent_module.path.append(symbol);

        let item_id = self
            .items
            .insert_if_not_exists(module_path.as_symbol_slice())
            .ok_or(ResolveError::already_defined(symbol))?;

        parent_module.elems.insert(
            symbol,
            ModuleElem {
                is_exported,
                kind: SymbolKind::Item(item_id),
            },
        );

        Ok(item_id)
    }

    pub fn insert_use<'a, P>(
        &mut self,
        parent_id: ModuleId,
        _is_exported: bool,
        path: P,
        _symbol: Option<Symbol>,
    ) -> ResolveResult<ItemId>
    where
        P: Into<ItemPath<'a>>,
    {
        let _parent_path = self.modules[&parent_id].path.as_path();

        let Some((_last, _others)) = path.into().as_symbol_slice().split_last() else {
            panic!("use path is empty");
        };

        todo!()
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
        let module_path = self.modules[&module_id].path.as_path();

        let path: ItemPath = path.into();
        let mut path_iter = path.as_symbol_slice().iter().peekable();
        let mut resolved_path = ItemPathBuf::default();

        while let Some(symbol) = path_iter.peek() {
            resolved_path = match **symbol {
                sym::KW_CRATE => module_path.first().into(),
                sym::KW_SUPER => resolved_path.try_pop().ok_or(ResolveError {
                    symbol: path.last(),
                    kind: ResolveErrorKind::TooManySuperKeywords,
                })?,
                sym::KW_SELF => module_path.to_path_buf(),
                _ => break,
            };

            path_iter.next();
        }

        for &symbol in path_iter {
            let current_module = self.get_module_by_path(&resolved_path)?;

            let current_item = current_module
                .elems
                .get(&symbol)
                .ok_or(ResolveError::not_found(symbol))?;

            if !current_item.is_exported && !module_path.starts_with(&current_module.path) {
                return Err(ResolveError::private(symbol));
            }

            resolved_path = resolved_path.append(symbol);
        }

        self.get_item_id_by_path(&resolved_path)
            .ok_or(ResolveError::not_found(resolved_path.last()))
    }

    #[inline]
    fn get_item_path(&self, item_id: ItemId) -> Option<ItemPath> {
        self.items.get(item_id).map(|path| path.into())
    }

    #[inline]
    fn get_item_id_by_path<'a, P>(&self, path: P) -> Option<ItemId>
    where
        P: Into<ItemPath<'a>>,
    {
        self.items.get_id(path.into().as_symbol_slice())
    }

    fn get_module_by_path<'a, P>(&self, path: P) -> ResolveResult<&Module>
    where
        P: Into<ItemPath<'a>>,
    {
        let path: ItemPath = path.into();

        let item_id = self
            .items
            .get_id(path.as_symbol_slice())
            .ok_or(ResolveError::not_found(path.last()))?;

        self.modules
            .get(&ModuleId(item_id.0))
            .ok_or(ResolveError::not_module(path.last()))
    }
}
