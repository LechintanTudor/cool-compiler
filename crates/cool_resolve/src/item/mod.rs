mod crates;
mod module;
mod path;

pub use self::crates::*;
pub use self::module::*;
pub use self::path::*;

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
        F: FnMut(&mut Self) -> I,
    {
        if self.modules[module_id].items.contains_key(&symbol) {
            return Err(ResolveError::SymbolAlreadyExists { symbol });
        }

        let item = add_item(self).into();

        let (crate_id, path) = {
            let item_id = self.modules[module_id].item_id;
            let (crate_id, crate_item_id) = self.items[item_id];
            let module_path = &self.crates[crate_id].paths[crate_item_id];

            let mut path = SmallVec::<Symbol, 8>::new();
            path.extend_from_slice(module_path);
            path.push(symbol);

            (crate_id, path)
        };

        let current_crate = &mut self.crates[crate_id];
        let crate_item_id = current_crate.paths.insert_slice(&path);
        let item_id = self.items.insert((crate_id, crate_item_id));
        let crate_item_id_copy = current_crate.items.push(item_id);
        let item_id_copy = self.item_defs.push(item);

        debug_assert_eq!(item_id, item_id_copy);
        debug_assert_eq!(crate_item_id, crate_item_id_copy);

        self.modules[module_id].items.insert(
            symbol,
            ModuleItem {
                is_exported,
                item_id,
            },
        );

        Ok(item_id)
    }
}
