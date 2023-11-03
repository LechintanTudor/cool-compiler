use crate::{tys, ItemId, ModuleId, ResolveContext, ResolveResult};
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
}
