use crate::{ItemId, ItemResult, ResolveContext};
use ahash::AHashMap;
use cool_arena::define_arena_index;
use cool_lexer::Symbol;
use std::ops::Index;

define_arena_index!(ModuleId);

#[derive(Clone, Debug)]
pub struct ModuleItem {
    pub item_id: ItemId,
    pub elems: AHashMap<Symbol, ModuleElem>,
}

impl ModuleItem {
    #[inline]
    pub fn new(item_id: ItemId) -> Self {
        Self {
            item_id,
            elems: Default::default(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ModuleElem {
    pub is_exported: bool,
    pub item_id: ItemId,
}

impl ResolveContext<'_> {
    pub fn add_root_module(&mut self, symbol: Symbol) -> ItemResult<ModuleId> {
        let item_id = self.add_path(&[symbol])?;
        Ok(self.modules.push(ModuleItem::new(item_id)))
    }

    pub fn add_module(
        &mut self,
        parent_module_id: ModuleId,
        is_exported: bool,
        symbol: Symbol,
    ) -> ItemResult<ModuleId> {
        let item_id = self.add_path(&[symbol])?;
        let module_id = self.modules.push(ModuleItem::new(item_id));

        self.modules[parent_module_id].elems.insert(
            symbol,
            ModuleElem {
                is_exported,
                item_id,
            },
        );

        Ok(module_id)
    }
}

impl Index<ModuleId> for ResolveContext<'_> {
    type Output = ModuleItem;

    #[inline]
    #[must_use]
    fn index(&self, index: ModuleId) -> &Self::Output {
        &self.modules[index]
    }
}
