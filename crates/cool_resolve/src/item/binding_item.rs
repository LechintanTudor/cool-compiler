use crate::{tys, Binding, ItemId, ItemResult, ModuleElem, ModuleId, Mutability, ResolveContext};
use cool_lexer::Symbol;

impl ResolveContext<'_> {
    pub fn add_global_binding(
        &mut self,
        parent_id: ModuleId,
        is_exported: bool,
        mutability: Mutability,
        symbol: Symbol,
    ) -> ItemResult<ItemId> {
        let item_id = self.add_path(&self.make_path(parent_id, symbol))?;
        let binding_id = self.bindings.push(Binding {
            symbol,
            mutability,
            ty_id: tys::infer,
        });

        self.items.insert(item_id, binding_id.into());
        self.modules[parent_id].elems.insert(
            symbol,
            ModuleElem {
                is_exported,
                item_id,
            },
        );

        Ok(item_id)
    }
}
