mod error;
mod item;
mod ty;

pub use self::error::*;
pub use self::item::*;
pub use self::ty::*;

use cool_collections::{Arena, VecMap};
use cool_lexer::Symbol;
use std::ops::Index;

#[derive(Debug)]
pub struct ResolveContext {
    items: VecMap<ItemId, Item>,
    crates: VecMap<CrateId, Crate>,
    modules: VecMap<ModuleId, Module>,
    ty_config: TyConfig,
    tys: Arena<TyId, TyKind>,
}

impl ResolveContext {
    pub fn new(ty_config: TyConfig) -> Self {
        let mut context = Self {
            items: VecMap::default(),
            crates: VecMap::default(),
            modules: VecMap::default(),
            ty_config,
            tys: Arena::default(),
        };

        context.add_crate(Symbol::insert("@builtins"));
        context
    }
}

impl Index<ItemId> for ResolveContext {
    type Output = Item;

    #[inline]
    #[must_use]
    fn index(&self, item_id: ItemId) -> &Self::Output {
        &self.items[item_id]
    }
}

impl Index<CrateId> for ResolveContext {
    type Output = Crate;

    #[inline]
    #[must_use]
    fn index(&self, crate_id: CrateId) -> &Self::Output {
        &self.crates[crate_id]
    }
}

impl Index<ModuleId> for ResolveContext {
    type Output = Module;

    #[inline]
    #[must_use]
    fn index(&self, module_id: ModuleId) -> &Self::Output {
        &self.modules[module_id]
    }
}

impl Index<TyId> for ResolveContext {
    type Output = TyKind;

    #[inline]
    #[must_use]
    fn index(&self, ty_id: TyId) -> &Self::Output {
        &self.tys[ty_id]
    }
}
