mod crates;
mod module;

pub use self::crates::*;
pub use self::module::*;

use crate::{BindingId, ResolveContext, ResolveError, ResolveResult, TyId};
use cool_collections::{define_index_newtype, SmallVec};
use cool_lexer::Symbol;
use derive_more::From;

define_index_newtype!(ItemId);

#[derive(Clone, Copy, PartialEq, Eq, Hash, From, Debug)]
pub enum Item {
    Binding(BindingId),
    Module(ModuleId),
    Ty(TyId),
}

impl Item {
    #[inline]
    pub const fn into_module(&self) -> ModuleId {
        match self {
            Self::Module(module_id) => *module_id,
            _ => panic!("Item is not a module"),
        }
    }
}

impl ResolveContext {
    pub(crate) fn add_item<I, F>(
        &mut self,
        module_id: ModuleId,
        is_exported: bool,
        symbol: Symbol,
        mut add_item: F,
    ) -> ResolveResult<ItemId>
    where
        I: Into<Item>,
        F: FnMut(&mut Self, ItemId, &[Symbol]) -> I,
    {
        if self.modules[module_id].items.contains_key(&symbol) {
            return Err(ResolveError::SymbolAlreadyExists { symbol });
        }

        let mut path: SmallVec<Symbol, 8> = SmallVec::new();
        path.extend_from_slice(&self.modules[module_id].path);
        path.push(symbol);

        let item = add_item(self, self.items.next_index(), &path).into();
        let item_id = self.items.push(item);

        let parent_module = &mut self.modules[module_id];
        parent_module
            .items
            .insert(*path.last().unwrap(), ModuleItem { is_exported, item });

        let parent_crate = &mut self.crates[parent_module.crate_id];
        let crate_item_id_1 = parent_crate.paths.insert_slice(&path);
        let crate_item_id_2 = parent_crate.items.push(item_id);
        debug_assert_eq!(crate_item_id_1, crate_item_id_2);

        Ok(item_id)
    }
}
