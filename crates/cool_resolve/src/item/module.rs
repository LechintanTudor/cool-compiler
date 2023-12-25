use crate::{ItemId, ResolveContext, ResolveError, ResolveResult};
use cool_collections::ahash::AHashMap;
use cool_collections::define_index_newtype;
use cool_lexer::Symbol;
use std::collections::hash_map::Entry;

define_index_newtype!(ModuleId);

impl ModuleId {
    pub const BUILTINS: Self = Self(0);
}

#[derive(Debug)]
pub struct Module {
    pub item_id: ItemId,
    pub items: AHashMap<Symbol, ModuleItem>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ModuleItem {
    pub is_exported: bool,
    pub item_id: ItemId,
}

impl ResolveContext {
    pub fn add_module(
        &mut self,
        module_id: ModuleId,
        is_exported: bool,
        symbol: Symbol,
    ) -> ResolveResult<ItemId> {
        self.add_item(module_id, is_exported, symbol, |context| {
            context.modules.push(Module {
                item_id: context.item_defs.next_index(),
                items: AHashMap::default(),
            })
        })
    }

    #[inline]
    pub fn add_import(
        &mut self,
        module_id: ModuleId,
        is_exported: bool,
        symbol: Symbol,
        item_id: ItemId,
    ) -> ResolveResult {
        let Entry::Occupied(mut entry) = self.modules[module_id].items.entry(symbol) else {
            return Err(ResolveError::SymbolAlreadyExists { symbol });
        };

        entry.insert(ModuleItem {
            is_exported,
            item_id,
        });

        Ok(())
    }
}
