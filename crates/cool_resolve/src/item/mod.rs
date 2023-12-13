mod crates;
mod module;
mod path;

pub use self::crates::*;
pub use self::module::*;
pub use self::path::*;

use crate::{ResolveContext, ResolveError, ResolveResult, TyId};
use cool_collections::{define_index_newtype, SmallVec};
use cool_lexer::Symbol;
use derive_more::From;

define_index_newtype!(ItemId);

#[derive(Clone, Copy, PartialEq, Eq, Hash, From, Debug)]
pub enum Item {
    Module(ModuleId),
    Ty(TyId),
}

impl ResolveContext {
    pub fn add_item(
        &mut self,
        module_id: ModuleId,
        is_exported: bool,
        symbol: Symbol,
        item: Item,
    ) -> ResolveResult<ItemId> {
        let parent = &mut self.modules[module_id];

        if parent.items.contains_key(&symbol) {
            return Err(ResolveError::SymbolAlreadyExists { symbol });
        }

        parent
            .items
            .insert(symbol, ModuleItem { is_exported, item });

        let path = {
            let mut path: SmallVec<Symbol, 8> = SmallVec::new();
            path.extend_from_slice(&parent.path);
            path.push(symbol);
            path
        };

        let crate_id = self.modules[module_id].crate_id;
        let parent_crate = &mut self.crates[crate_id];

        let item_id = parent_crate.paths.insert_slice(&path);
        let actual_item_id = parent_crate.items.push(item);
        debug_assert_eq!(item_id, actual_item_id);

        Ok(item_id)
    }
}
