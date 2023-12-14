use crate::{Item, ItemId, Module, ModuleId, ResolveContext, ResolveError, ResolveResult};
use cool_collections::ahash::AHashMap;
use cool_collections::smallvec::smallvec;
use cool_collections::{define_index_newtype, Arena, VecMap};
use cool_lexer::Symbol;

define_index_newtype!(CrateId);

impl CrateId {
    pub const BUILTINS: Self = Self::new(0);

    #[inline]
    #[must_use]
    pub const fn as_module_id(&self) -> ModuleId {
        ModuleId::new(self.get())
    }
}

#[derive(Debug)]
pub struct Crate {
    pub name: Symbol,
    pub deps: AHashMap<Symbol, CrateId>,
    pub paths: Arena<ItemId, [Symbol]>,
    pub items: VecMap<ItemId, Item>,
}

impl ResolveContext {
    pub fn add_crate(&mut self, name: Symbol) -> CrateId {
        let crate_id = self.crates.push(Crate {
            name,
            deps: AHashMap::default(),
            paths: Arena::default(),
            items: VecMap::default(),
        });

        let module_id = self.modules.push(Module {
            crate_id,
            path: smallvec![],
            items: AHashMap::default(),
        });

        assert_eq!(crate_id.get(), module_id.get());
        crate_id
    }

    pub fn add_dep(&mut self, crate_id: CrateId, symbol: Symbol, dep_id: CrateId) -> ResolveResult {
        let parent_crate = &mut self.crates[crate_id];

        if parent_crate.deps.contains_key(&symbol) {
            return Err(ResolveError::SymbolAlreadyExists { symbol });
        }

        parent_crate.deps.insert(symbol, dep_id);
        Ok(())
    }
}
