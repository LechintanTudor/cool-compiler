mod item_error;
mod item_kind;
mod module_item;

pub use self::item_error::*;
pub use self::item_kind::*;
pub use self::module_item::*;

use crate::ResolveContext;
use cool_arena::define_arena_index;
use cool_lexer::Symbol;
use std::ops::Index;

define_arena_index!(ItemId);

impl ResolveContext<'_> {
    pub fn add_path(&mut self, path: &[Symbol]) -> ItemResult<ItemId> {
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
}

impl Index<ItemId> for ResolveContext<'_> {
    type Output = ItemKind;

    #[inline]
    #[must_use]
    fn index(&self, id: ItemId) -> &Self::Output {
        &self.items[&id]
    }
}
