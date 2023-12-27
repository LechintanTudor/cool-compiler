use crate::{ItemId, ResolveContext, ResolveResult};
use cool_collections::ahash::AHashMap;
use cool_collections::define_index_newtype;
use cool_lexer::Symbol;

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

    pub fn add_import(
        &mut self,
        module_id: ModuleId,
        is_exported: bool,
        path: &[Symbol],
        symbol: Symbol,
    ) -> ResolveResult<ItemId> {
        let item = self[self.resolve_path(module_id, path)?];
        self.add_item(module_id, is_exported, symbol, |_| item)
    }
}
