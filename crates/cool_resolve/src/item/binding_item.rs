use crate::{tys, Binding, ItemId, ItemResult, ModuleId, Mutability, ResolveContext};
use cool_lexer::Symbol;

impl ResolveContext<'_> {
    pub fn add_global_binding(
        &mut self,
        module_id: ModuleId,
        is_exported: bool,
        mutability: Mutability,
        symbol: Symbol,
    ) -> ItemResult<ItemId> {
        let item_id = self.add_path(module_id, symbol)?;
        let binding_id = self.bindings.push(Binding {
            symbol,
            mutability,
            ty_id: tys::infer,
        });
        self.add_item(module_id, is_exported, symbol, item_id, binding_id);
        Ok(item_id)
    }
}
