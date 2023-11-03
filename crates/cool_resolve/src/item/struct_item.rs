use crate::{ItemId, ModuleId, ResolveContext, ResolveResult, StructTy};
use cool_lexer::Symbol;

impl ResolveContext<'_> {
    pub fn add_struct(
        &mut self,
        module_id: ModuleId,
        is_exported: bool,
        symbol: Symbol,
    ) -> ResolveResult<ItemId> {
        let item_id = self.add_path(module_id, symbol)?;
        let ty_id = self.add_ty(StructTy { item_id });
        self.add_item(module_id, is_exported, symbol, item_id, ty_id);
        Ok(item_id)
    }
}
