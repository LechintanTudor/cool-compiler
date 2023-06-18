use crate::{
    DefineError, DefineErrorKind, DefineResult, EnumTy, ItemId, ItemKind, ModuleId, ResolveContext,
    ResolveResult, TyId,
};
use cool_lexer::Symbol;
use std::collections::HashSet;
use std::sync::Arc;

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
    ) -> DefineResult<()>
    where
        V: IntoIterator<Item = Symbol>,
    {
        let ItemKind::Ty(item_ty_id) = &mut self.items[item_id] else {
            panic!("item is not a type");
        };

        assert!(item_ty_id.is_infer());

        let variants = variants.into_iter().collect::<Arc<[Symbol]>>();
        let mut used_variants = HashSet::<Symbol>::default();

        for &variant in variants.iter() {
            if !used_variants.insert(variant) {
                return Err(DefineError {
                    path: self.paths[item_id].into(),
                    kind: DefineErrorKind::EnumHasDuplicatedVariant { variant },
                });
            }
        }

        let storage = match storage {
            Some(storage) => {
                if !storage.is_int() {
                    return Err(DefineError {
                        path: self.paths[item_id].into(),
                        kind: DefineErrorKind::EnumHasInvalidStorage { storage },
                    });
                }

                storage
            }
            None => self.tys.get_or_insert_value(EnumTy::DEFAULT_STORAGE),
        };

        let ty_id = self.tys.get_or_insert_value(EnumTy {
            item_id,
            storage,
            variants,
        });

        *item_ty_id = ty_id;
        Ok(())
    }
}
