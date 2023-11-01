mod alias_item;
mod binding_item;
mod item_error;
mod item_kind;
mod module_item;
mod struct_item;

pub use self::alias_item::*;
pub use self::binding_item::*;
pub use self::item_error::*;
pub use self::item_kind::*;
pub use self::module_item::*;
pub use self::struct_item::*;

use crate::ResolveContext;
use cool_collections::define_index_newtype;
use cool_lexer::Symbol;
use smallvec::SmallVec;
use std::ops::Index;

define_index_newtype!(ItemId);

impl ResolveContext<'_> {
    pub fn add_path(&mut self, module_id: ModuleId, symbol: Symbol) -> ItemResult<ItemId> {
        let module = &self.modules[module_id];
        let mut path: SmallVec<[Symbol; 12]> = self.paths[module.item_id].into();
        path.push(symbol);
        self.add_raw_path(&path)
    }

    pub fn add_raw_path(&mut self, path: &[Symbol]) -> ItemResult<ItemId> {
        let item_id = self.paths.insert_slice(path);

        if self.items.contains_key(&item_id) {
            return Err(ItemError {
                item_id,
                kind: ItemErrorKind::AlreadyExists,
            });
        }

        Ok(item_id)
    }

    pub fn get_path(&self, item_id: ItemId) -> ItemResult<&[Symbol]> {
        self.paths.get(item_id).ok_or(ItemError {
            item_id,
            kind: ItemErrorKind::NotFound,
        })
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
