mod alias_item;
mod binding_item;
mod import;
mod item_kind;
mod module_item;
mod resolve;
mod struct_item;

pub use self::alias_item::*;
pub use self::binding_item::*;
pub use self::import::*;
pub use self::item_kind::*;
pub use self::module_item::*;
pub use self::resolve::*;
pub use self::struct_item::*;

use crate::{ResolveContext, ResolveError, ResolveResult};
use cool_collections::define_index_newtype;
use cool_lexer::Symbol;
use smallvec::SmallVec;
use std::ops::Index;

define_index_newtype!(ItemId);

impl ResolveContext<'_> {
    #[inline]
    #[must_use]
    pub fn get_path(&self, item_id: ItemId) -> &[Symbol] {
        &self.paths[item_id]
    }

    #[inline]
    #[must_use]
    pub fn get_module_path(&self, module_id: ModuleId) -> &[Symbol] {
        &self.paths[self.modules[module_id].item_id]
    }

    fn add_path(&mut self, module_id: ModuleId, symbol: Symbol) -> ResolveResult<ItemId> {
        let module = &self.modules[module_id];
        let mut path: SmallVec<[Symbol; 12]> = self.paths[module.item_id].into();
        path.push(symbol);
        self.add_raw_path(&path)
    }

    fn add_raw_path(&mut self, path: &[Symbol]) -> ResolveResult<ItemId> {
        let item_id = self.paths.insert_slice(path);

        if self.items.contains_key(&item_id) {
            return Err(ResolveError::SymbolAlreadyExists {
                symbol: *path.last().unwrap(),
            });
        }

        Ok(item_id)
    }

    fn add_item<I>(
        &mut self,
        module_id: ModuleId,
        is_exported: bool,
        symbol: Symbol,
        item_id: ItemId,
        item: I,
    ) where
        I: Into<ItemKind>,
    {
        self.items.insert(item_id, item.into());
        self.modules[module_id].elems.insert(
            symbol,
            ModuleElem {
                is_exported,
                item_id,
            },
        );
    }
}

impl Index<ItemId> for ResolveContext<'_> {
    type Output = ItemKind;

    #[inline]
    #[must_use]
    fn index(&self, id: ItemId) -> &Self::Output {
        &self.items[&id]
    }
}
