use crate::resolve::{ItemId, Module, ModuleId, ResolveTable, ScopeId, SymbolKind};
use crate::{ItemPath, ItemPathBuf};
use cool_lexer::symbols::{sym, Symbol};

impl ResolveTable {
    pub fn get_first_module_id(&self, mut scope_id: ScopeId) -> ModuleId {
        loop {
            match scope_id {
                ScopeId::Module(module_id) => break module_id,
                ScopeId::Frame(frame_id) => {
                    scope_id = self.frames[frame_id].parent_id;
                }
            }
        }
    }

    #[inline]
    pub fn get_module_by_id2(&self, item_id: ItemId) -> Option<&Module> {
        self.modules.get(&ModuleId(item_id.0))
    }

    pub fn get_module_by_path2<'a, P>(&self, path: P) -> Option<&Module>
    where
        P: Into<ItemPath<'a>>,
    {
        let path: ItemPath = path.into();
        let item_id = self.items.get_id(path.as_symbol_slice())?;
        self.modules.get(&ModuleId(item_id.0))
    }

    pub fn resolve_module_access(&self) {
        todo!()
    }

    pub fn resolve_symbol(&self, mut scope_id: ScopeId, symbol: Symbol) -> SymbolKind {
        loop {
            match scope_id {
                ScopeId::Frame(frame_id) => {
                    let frame = &self.frames[frame_id];

                    match frame.bindings.get(&symbol) {
                        Some(binding_id) => return SymbolKind::Binding(*binding_id),
                        None => scope_id = frame.parent_id,
                    }
                }
                ScopeId::Module(module_id) => {
                    let module = &self.modules[&module_id];

                    match module.elems.get(&symbol) {
                        Some(elem) => return elem.kind,
                        None => break,
                    }
                }
            }
        }

        // TODO: Check global scope

        let builtins_module = &self.modules[&ModuleId::for_builtins()];
        builtins_module.elems.get(&symbol).unwrap().kind
    }

    pub fn resolve_path_as_item<'a, P>(&self, scope_id: ScopeId, path: P) -> Option<ItemId>
    where
        P: Into<ItemPath<'a>>,
    {
        let path: ItemPath = path.into();
        let module_id = self.get_first_module_id(scope_id);
        let module_path = ItemPath::from(&self.items[module_id.as_item_id()]);

        let mut resolved_path = if path.starts_with_crate() {
            ItemPathBuf::from(module_path.first())
        } else if path.starts_with_self_or_super() {
            module_path.to_path_buf()
        } else {
            todo!()
        };

        for symbol in path.as_symbol_slice() {
            resolved_path = match *symbol {
                sym::KW_SELF | sym::KW_CRATE => continue,
                sym::KW_SUPER => resolved_path.pop(),
                symbol => resolved_path.append(symbol),
            };
        }

        let Some((final_symbol, module_symbols)) = resolved_path.as_symbol_slice().split_last() else {
            panic!("resolved path is empty");
        };

        let mut current_module = match self.get_module_by_path2(&module_symbols[..1]) {
            Some(module) => module,
            None => return None,
        };

        for symbol in &module_symbols[1..] {
            let Some(module_elem) = current_module.elems.get(symbol) else {
                return None;
            };

            if !module_elem.is_exported && !module_path.starts_with(&current_module.path) {
                panic!("tried to import private symbol");
            };

            let next_module = match self.get_module_by_id2(module_elem.kind.as_item_id().unwrap()) {
                Some(module) => module,
                None => return None,
            };

            current_module = next_module;
        }

        let Some(elem) = current_module.elems.get(final_symbol) else {
            return None;
        };

        if !elem.is_exported && !module_path.starts_with(&current_module.path) {
            panic!("tried to import private symbol");
        }

        Some(elem.kind.as_item_id().unwrap())
    }

    pub fn resolve_local_path_as_item<'a, P>(
        &self,
        _scope_id: ScopeId,
        _path: P,
    ) -> Option<ItemId> {
        todo!()
    }
}
