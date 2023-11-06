use crate::{tys, ItemId, ItemKind, ModuleId, ResolveContext, ResolveResult, TyId};
use cool_lexer::Symbol;

impl ResolveContext<'_> {
    pub fn add_alias(
        &mut self,
        module_id: ModuleId,
        is_exported: bool,
        symbol: Symbol,
    ) -> ResolveResult<ItemId> {
        let item_id = self.add_path(module_id, symbol)?;
        self.add_item(module_id, is_exported, symbol, item_id, tys::infer);
        Ok(item_id)
    }

    pub fn update_alias(&mut self, item_id: ItemId, ty_id: TyId) {
        match self.items.get_mut(&item_id).unwrap() {
            ItemKind::Ty(item_ty_id) => *item_ty_id = ty_id,
            _ => panic!("Item is not an alias"),
        }
    }
}
