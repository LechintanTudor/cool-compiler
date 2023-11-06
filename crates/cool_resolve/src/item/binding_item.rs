use crate::{tys, Binding, ItemId, ModuleId, ResolveContext, ResolveResult};
use cool_lexer::Symbol;

impl ResolveContext<'_> {
    pub fn add_global_binding(
        &mut self,
        module_id: ModuleId,
        is_exported: bool,
        is_mutable: bool,
        symbol: Symbol,
    ) -> ResolveResult<ItemId> {
        let item_id = self.add_path(module_id, symbol)?;
        let binding_id = self.bindings.push(Binding {
            symbol,
            is_mutable,
            ty_id: tys::infer,
        });
        self.add_item(module_id, is_exported, symbol, item_id, binding_id);
        Ok(item_id)
    }
}
