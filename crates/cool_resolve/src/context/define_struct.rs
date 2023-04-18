use crate::{
    ItemId, ItemKind, ModuleElem, ModuleId, ResolveContext, ResolveError, ResolveResult,
    StructHasInfiniteSize, StructTy, TyId, TyKind,
};
use cool_collections::id_newtype;
use cool_lexer::symbols::Symbol;
use std::collections::VecDeque;

id_newtype!(StructId);

impl ResolveContext {
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
        let ty_id = self.tys.get_or_insert(TyKind::Struct(struct_id));
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

    // TODO: Separate error type for ty definitions
    pub fn define_struct(
        &mut self,
        item_id: ItemId,
        struct_ty: StructTy,
    ) -> Result<bool, StructHasInfiniteSize> {
        let struct_ty_id = self.items[item_id].as_ty_id().expect("item is not a type");

        let struct_id = self.tys[struct_ty_id]
            .as_struct_id()
            .expect("type is not a struct");

        for ty_id in struct_ty.fields.values() {
            match self.ty_contains_ty(*ty_id, struct_ty_id) {
                Some(true) => {
                    return Err(StructHasInfiniteSize {
                        path: self.paths[item_id].into(),
                    })
                }
                Some(false) => (),
                None => return Ok(false),
            }
        }

        self.struct_tys[struct_id] = Some(struct_ty);
        Ok(true)
    }

    fn ty_contains_ty(&self, haysack_ty_id: TyId, needle_ty_id: TyId) -> Option<bool> {
        let mut tys_to_check = VecDeque::<TyId>::new();
        tys_to_check.push_back(haysack_ty_id);

        while let Some(ty_id) = tys_to_check.pop_front() {
            if ty_id == needle_ty_id {
                return Some(true);
            }

            match &self.tys[ty_id] {
                TyKind::Tuple(tuple_ty) => tys_to_check.extend(tuple_ty.elems.iter()),
                TyKind::Struct(struct_id) => {
                    let Some(struct_ty) = &self.struct_tys[*struct_id] else {
                        return None;
                    };

                    tys_to_check.extend(struct_ty.fields.values());
                }
                _ => (),
            }
        }

        Some(false)
    }
}
