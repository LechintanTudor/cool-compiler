use crate::{ItemId, ResolveContext};
use cool_collections::{define_index_newtype, Arena, VecMap};
use cool_lexer::Symbol;

define_index_newtype!(CrateId);
define_index_newtype!(CrateItemId);

impl CrateId {
    pub const BUILTINS: Self = Self::new(0);
}

#[derive(Debug)]
pub struct Crate {
    pub name: Symbol,
    pub paths: Arena<CrateItemId, [Symbol]>,
    pub items: VecMap<CrateItemId, ItemId>,
}

impl Crate {
    pub fn new(name: Symbol) -> Self {
        Self {
            name,
            paths: Arena::default(),
            items: VecMap::default(),
        }
    }

    #[inline]
    pub fn add_item(&mut self, path: &[Symbol], item_id: ItemId) {
        let crate_item_id_1 = self.paths.insert_slice(path);
        let crate_item_id_2 = self.items.push(item_id);
        debug_assert_eq!(crate_item_id_1, crate_item_id_2);
    }
}

impl ResolveContext {
    pub fn add_crate(&mut self, name: Symbol) -> CrateId {
        let crate_id = self.crates.push(Crate::new(name));
        self.items.push(crate_id.into());
        crate_id
    }
}
