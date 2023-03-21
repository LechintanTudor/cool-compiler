use crate::consts::itm;
use crate::{
    Binding, BindingId, Frame, FrameId, ItemId, ItemPath, ItemPathBuf, Module, ModuleElem,
    ModuleId, ResolveTable, ScopeId, SymbolKind,
};
use cool_collections::IdIndexedVec;
use cool_lexer::symbols::{sym, Symbol};

impl ResolveTable {
    pub fn with_builtins() -> Self {
        let mut resolve = Self::default();
        assert_eq!(
            resolve.add_root_module(sym::EMPTY),
            ModuleId::for_builtins()
        );
        itm::add_builtins(&mut resolve);
        resolve
    }

    pub fn add_builtin_item(&mut self, item_id: ItemId, symbol: Symbol) {
        assert_eq!(
            self.add_item_to_module(ModuleId::for_builtins(), true, symbol),
            item_id,
        );
    }

    pub fn add_root_module(&mut self, symbol: Symbol) -> ModuleId {
        let module_id = self
            .items
            .insert_if_not_exists(&[symbol])
            .map(|id| ModuleId(id.0))
            .expect("module already exists");

        self.modules.insert(
            module_id,
            Module {
                path: ItemPathBuf::from(symbol),
                elems: Default::default(),
            },
        );

        module_id
    }

    pub fn add_item_to_module(
        &mut self,
        module_id: ModuleId,
        is_exported: bool,
        symbol: Symbol,
    ) -> ItemId {
        let module = self.modules.get_mut(&module_id).unwrap();
        let item_path = module.path.append(symbol);
        let item_id = self
            .items
            .insert_if_not_exists(item_path.as_symbol_slice())
            .unwrap();

        module.elems.insert(
            symbol,
            ModuleElem {
                is_exported,
                kind: SymbolKind::Item(item_id),
            },
        );

        item_id
    }

    pub fn add_module_to_module(
        &mut self,
        module_id: ModuleId,
        is_exported: bool,
        symbol: Symbol,
    ) -> ModuleId {
        let (child_module_path, child_module_id) = {
            let module = self.modules.get_mut(&module_id).unwrap();
            let item_path = module.path.append(symbol);
            let item_id = self
                .items
                .insert_if_not_exists(item_path.as_symbol_slice())
                .unwrap();

            module.elems.insert(
                symbol,
                ModuleElem {
                    is_exported,
                    kind: SymbolKind::Item(item_id),
                },
            );

            (item_path, ModuleId(item_id.0))
        };

        self.modules.insert(
            child_module_id,
            Module {
                path: child_module_path,
                elems: Default::default(),
            },
        );

        child_module_id
    }

    pub fn add_binding_to_module(
        &mut self,
        module_id: ModuleId,
        is_exported: bool,
        is_mutable: bool,
        symbol: Symbol,
    ) -> BindingId {
        let module = self.modules.get_mut(&module_id).unwrap();
        assert!(!module.elems.contains_key(&symbol));

        let binding_id = self.bindings.push(Binding { is_mutable });

        module.elems.insert(
            symbol,
            ModuleElem {
                is_exported,
                kind: SymbolKind::Binding(binding_id),
            },
        );

        binding_id
    }

    pub fn add_use_to_module<'a, P>(
        &mut self,
        _module_id: ModuleId,
        _is_exported: bool,
        _item_path: P,
        _symbol: Option<Symbol>,
    ) -> bool
    where
        P: Into<ItemPath<'a>>,
    {
        todo!()
    }

    pub fn add_frame(&mut self, parent_id: ScopeId) -> FrameId {
        self.frames.push(Frame {
            parent_id,
            bindings: Default::default(),
        })
    }

    pub fn add_binding_to_frame(
        &mut self,
        frame_id: FrameId,
        is_mutable: bool,
        symbol: Symbol,
    ) -> BindingId {
        let binding_id = self.bindings.push(Binding { is_mutable });

        let exists = self
            .frames
            .get_mut(frame_id)
            .unwrap()
            .bindings
            .insert_if_not_exists(symbol, binding_id);

        assert!(!exists);
        binding_id
    }

}

impl Default for ResolveTable {
    fn default() -> Self {
        Self {
            items: Default::default(),
            modules: Default::default(),
            frames: IdIndexedVec::new(Frame {
                parent_id: ScopeId::Module(ModuleId::for_builtins()),
                bindings: Default::default(),
            }),
            bindings: IdIndexedVec::new(Binding { is_mutable: false }),
        }
    }
}
