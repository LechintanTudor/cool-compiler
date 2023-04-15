use crate::{
    ItemId, ItemKind, ModuleElem, ModuleId, ResolveError, ResolveResult, ResolveTable, TyId,
};
use cool_collections::{id_newtype, SmallVecMap};
use cool_lexer::symbols::Symbol;

id_newtype!(StructId);

#[derive(Clone, Debug)]
pub struct StructTy {
    fields: SmallVecMap<Symbol, TyId, 2>,
}

impl StructTy {
    #[inline]
    pub fn builder() -> StructTyBuilder {
        StructTyBuilder::default()
    }
}

#[derive(Default, Debug)]
pub struct StructTyBuilder {
    fields: SmallVecMap<Symbol, TyId, 2>,
}

impl StructTyBuilder {
    #[inline]
    pub fn add_field(&mut self, symbol: Symbol, ty_id: TyId) {
        assert!(self.fields.insert_if_not_exists(symbol, ty_id));
    }

    #[inline]
    pub fn build(self) -> StructTy {
        StructTy {
            fields: self.fields,
        }
    }
}

impl ResolveTable {
    pub fn declare_struct(
        &mut self,
        module_id: ModuleId,
        is_exported: bool,
        symbol: Symbol,
    ) -> ResolveResult<ItemId> {
        let module = &mut self.modules[module_id];
        let item_path = module.path.append(symbol);

        let item_id = self
            .paths
            .insert_if_not_exists(item_path.as_symbol_slice())
            .ok_or(ResolveError::already_defined(symbol))?;

        let struct_id = self.struct_tys.push(None);
        let ty_id = self.tys.mk_struct(struct_id);
        self.items.push_checked(item_id, ItemKind::Ty(ty_id));

        module.elems.insert(
            symbol,
            ModuleElem {
                is_exported,
                item_id,
            },
        );

        Ok(item_id)
    }
}
