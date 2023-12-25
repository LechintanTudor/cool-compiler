use crate::{ItemId, Module, ModuleId, ResolveContext, ResolveError, ResolveResult};
use cool_collections::ahash::AHashMap;
use cool_collections::{define_index_newtype, Arena, VecMap};
use cool_lexer::Symbol;
use std::collections::hash_map::Entry;

define_index_newtype!(CrateId);
define_index_newtype!(CrateItemId);

impl CrateId {
    pub const BUILTINS: Self = Self::new(0);

    #[inline]
    #[must_use]
    pub const fn as_module_id(&self) -> ModuleId {
        ModuleId::new(self.get())
    }
}

impl CrateItemId {
    pub const ROOT: Self = Self::new(0);
}

#[derive(Debug)]
pub struct Crate {
    pub name: Symbol,
    pub deps: AHashMap<Symbol, CrateId>,
    pub paths: Arena<CrateItemId, [Symbol]>,
    pub items: VecMap<CrateItemId, ItemId>,
}

impl ResolveContext {
    pub fn add_crate(&mut self, name: Symbol) -> CrateId {
        assert_eq!(self.crates.len(), self.modules.len());

        let crate_id = self.crates.push(Crate {
            name,
            deps: AHashMap::default(),
            paths: Arena::default(),
            items: VecMap::default(),
        });

        let item_id = self.items.insert((crate_id, CrateItemId::ROOT));

        let module_id = self.modules.push(Module {
            item_id,
            items: AHashMap::default(),
        });

        debug_assert_eq!(crate_id.get(), module_id.get());

        let current_crate = &mut self.crates[crate_id];
        let crate_item_id = current_crate.paths.insert_slice(&[]);
        let crate_item_id_copy = current_crate.items.push(item_id);
        debug_assert_eq!(crate_item_id, crate_item_id_copy);

        let item_id_copy = self.item_defs.push(module_id.into());
        debug_assert_eq!(item_id, item_id_copy);

        crate_id
    }

    pub fn add_dep(&mut self, crate_id: CrateId, symbol: Symbol, dep_id: CrateId) -> ResolveResult {
        let Entry::Vacant(entry) = self.crates[crate_id].deps.entry(symbol) else {
            return Err(ResolveError::SymbolAlreadyExists { symbol });
        };

        entry.insert(dep_id);
        Ok(())
    }
}
