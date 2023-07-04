use crate::{
    EmptyStructTy, ItemId, ItemKind, ModuleElem, ModuleId, ResolveContext, ResolveError,
    ResolveErrorKind, ResolveResult, StructTy, TyDef, TyId, TyResult, ValueTy,
};
use cool_lexer::Symbol;

impl ResolveContext {
    pub fn declare_struct(
        &mut self,
        module_id: ModuleId,
        is_exported: bool,
        symbol: Symbol,
        has_body: bool,
    ) -> ResolveResult<ItemId> {
        let module = &mut self.modules[module_id];
        let item_path = module.child_path(symbol);

        let item_id = self
            .paths
            .insert_slice_if_not_exists(item_path.as_symbol_slice())
            .map(ItemId::from)
            .ok_or(ResolveError {
                symbol,
                kind: ResolveErrorKind::SymbolAlreadyDefined,
            })?;

        let ty = if has_body {
            ValueTy::from(StructTy { item_id })
        } else {
            ValueTy::from(EmptyStructTy { item_id })
        };

        let ty_id = self.tys.insert_value(ty);
        self.items.insert(item_id, ItemKind::Ty(ty_id));

        module.elems.insert(
            symbol,
            ModuleElem {
                is_exported,
                item_id,
            },
        );

        Ok(item_id)
    }

    pub fn define_struct<F>(&mut self, item_id: ItemId, fields: F) -> TyResult<&TyDef>
    where
        F: IntoIterator<Item = (Symbol, TyId)>,
    {
        let struct_ty_id = self.items[&item_id]
            .as_ty_id()
            .expect("item is not a struct");

        self.tys.define_struct(struct_ty_id, fields)
    }
}
