use crate::{Item, ItemId, Module, ModuleId, ResolveContext};
use cool_collections::ahash::AHashMap;
use cool_collections::smallvec::smallvec;
use cool_collections::{define_index_newtype, Arena, VecMap};
use cool_lexer::Symbol;

define_index_newtype!(CrateId);

impl CrateId {
    pub const BUILTINS: Self = Self::new(0);
}

#[derive(Debug)]
pub struct Crate {
    pub name: Symbol,
    pub module_id: ModuleId,
    pub paths: Arena<ItemId, [Symbol]>,
    pub items: VecMap<ItemId, Item>,
}

impl ResolveContext {
    pub fn add_crate(&mut self, name: Symbol) -> CrateId {
        let crate_id = self.crates.next_index();

        let module_id = self.modules.push(Module {
            crate_id,
            path: smallvec![],
            items: AHashMap::default(),
        });

        self.crates.push(Crate {
            name,
            module_id,
            paths: Arena::default(),
            items: VecMap::default(),
        });

        crate_id
    }
}
