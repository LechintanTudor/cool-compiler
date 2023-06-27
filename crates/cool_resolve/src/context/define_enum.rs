use crate::{ItemId, ModuleId, ResolveContext, ResolveResult, TyId, TyResult};
use cool_lexer::Symbol;

impl ResolveContext {
    pub fn declare_enum(
        &mut self,
        module_id: ModuleId,
        is_exported: bool,
        symbol: Symbol,
    ) -> ResolveResult<ItemId> {
        self.declare_alias(module_id, is_exported, symbol)
    }

    pub fn define_enum<V>(
        &mut self,
        item_id: ItemId,
        storage: Option<TyId>,
        variants: V,
    ) -> TyResult
    where
        V: IntoIterator<Item = Symbol>,
    {
        todo!()
    }
}
