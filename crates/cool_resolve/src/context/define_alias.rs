use crate::{
    ItemId, ItemKind, ModuleElem, ModuleId, ResolveContext, ResolveError, ResolveErrorKind,
    ResolveResult, TyId,
};
use cool_lexer::Symbol;

impl ResolveContext {
    pub fn declare_alias(
        &mut self,
        module_id: ModuleId,
        is_exported: bool,
        symbol: Symbol,
    ) -> ResolveResult<ItemId> {
        let module = &mut self.modules[module_id];
        let item_path = module.path.append(symbol);

        let item_id = self
            .paths
            .insert_slice_if_not_exists(item_path.as_symbol_slice())
            .ok_or(ResolveError {
                symbol,
                kind: ResolveErrorKind::SymbolAlreadyDefined,
            })?;

        self.items
            .push_checked(item_id, ItemKind::Ty(self.tys.consts().infer));

        module.elems.insert(
            symbol,
            ModuleElem {
                is_exported,
                item_id,
            },
        );

        Ok(item_id)
    }

    pub fn define_alias(&mut self, item_id: ItemId, resolved_ty_id: TyId) {
        let item = &mut self.items[item_id];

        let ItemKind::Ty(alias_ty_id) = item else {
            panic!("item is not a type alias");
        };

        assert!(alias_ty_id.is_infer());
        *alias_ty_id = resolved_ty_id;
    }
}
