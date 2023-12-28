mod ty_config;
mod ty_consts;
mod ty_def;
mod ty_factory;
mod ty_id;
mod ty_kind;

pub use self::ty_config::*;
pub use self::ty_consts::*;
pub use self::ty_def::*;
pub use self::ty_id::*;
pub use self::ty_kind::*;

use crate::{Item, ItemId, ModuleId, ResolveContext, ResolveResult};
use cool_lexer::Symbol;

impl ResolveContext {
    pub(crate) fn add_ty(&mut self, ty_kind: TyKind) -> TyId {
        let ty_id_1 = self.tys.insert(ty_kind);

        if !self.ty_defs.contains_index(ty_id_1) {
            let ty_id_2 = self.ty_defs.push(None);
            debug_assert_eq!(ty_id_1, ty_id_2);
        }

        ty_id_1
    }

    #[inline]
    pub fn add_alias(
        &mut self,
        module_id: ModuleId,
        is_exported: bool,
        symbol: Symbol,
    ) -> ResolveResult<ItemId> {
        self.add_item(module_id, is_exported, symbol, |_| tys::infer)
    }

    pub fn define_alias(&mut self, item_id: ItemId, ty_id: TyId) {
        let Item::Ty(old_ty_id) = &mut self.item_defs[item_id] else {
            panic!("Item is not an alias");
        };

        assert_eq!(*old_ty_id, tys::infer, "Alias is already defined");
        *old_ty_id = ty_id;
    }

    pub fn add_struct(
        &mut self,
        module_id: ModuleId,
        is_exported: bool,
        symbol: Symbol,
    ) -> ResolveResult<ItemId> {
        self.add_item(module_id, is_exported, symbol, |context| {
            context.add_struct_ty(context.item_defs.next_index())
        })
    }
}
