use crate::{ItemId, ItemResult, ResolveContext};
use ahash::AHashMap;
use cool_collections::define_index_newtype;
use cool_lexer::Symbol;
use std::ops::Index;

define_index_newtype!(ModuleId);

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
        let module_id = self.modules.push(ModuleItem::new(item_id));
        self.items.insert(item_id, module_id.into());
        Ok(module_id)
    }

    pub fn add_module(
        &mut self,
        parent_id: ModuleId,
        is_exported: bool,
        symbol: Symbol,
    ) -> ItemResult<ModuleId> {
        let item_id = self.add_path(&self.make_path(parent_id, symbol))?;
        let module_id = self.modules.push(ModuleItem::new(item_id));

        self.items.insert(item_id, module_id.into());
        self.modules[parent_id].elems.insert(
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
    fn index(&self, module_id: ModuleId) -> &Self::Output {
        &self.modules[module_id]
    }
}
