use crate::{CrateId, Item, ItemId, ResolveContext, ResolveResult};
use cool_collections::ahash::AHashMap;
use cool_collections::{define_index_newtype, SmallVec};
use cool_lexer::Symbol;

define_index_newtype!(ModuleId);

impl ModuleId {
    pub const BUILTINS: Self = Self(0);
}

#[derive(Debug)]
pub struct Module {
    pub crate_id: CrateId,
    pub path: SmallVec<Symbol, 4>,
    pub items: AHashMap<Symbol, ModuleItem>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ModuleItem {
    pub is_exported: bool,
    pub item: Item,
}

impl ResolveContext {
    pub fn add_module(
        &mut self,
        module_id: ModuleId,
        is_exported: bool,
        symbol: Symbol,
    ) -> ResolveResult<ItemId> {
        self.add_item(module_id, is_exported, symbol, |context, _, path| {
            context.modules.push(Module {
                crate_id: context.modules[module_id].crate_id,
                path: path.into(),
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
        item: Item,
    ) -> ResolveResult {
        self.add_item(module_id, is_exported, symbol, |_, _, _| item)?;
        Ok(())
    }
}
