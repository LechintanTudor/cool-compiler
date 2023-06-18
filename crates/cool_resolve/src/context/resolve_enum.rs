use crate::{IntTy, ItemId, ModuleId, ResolveContext, ResolveResult};
use cool_lexer::Symbol;

impl ResolveContext {
    pub fn declare_enum(
        &mut self,
        module_id: ModuleId,
        is_exported: bool,
        symbol: Symbol,
    ) -> ResolveResult<ItemId> {
        todo!()
    }

    pub fn define_enum<V>(&mut self, item_id: ItemId, storage: Option<IntTy>, variants: V)
    where
        V: IntoIterator<Item = Symbol>,
    {
        todo!()
    }
}
