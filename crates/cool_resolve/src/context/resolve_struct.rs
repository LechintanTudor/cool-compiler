use crate::{
    Field, ItemId, ItemKind, ModuleElem, ModuleId, ResolveContext, ResolveError, ResolveErrorKind,
    ResolveResult, StructHasInfiniteSize, TyId, ValueTy,
};
use cool_lexer::Symbol;
use smallvec::SmallVec;

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
            .ok_or(ResolveError {
                symbol,
                kind: ResolveErrorKind::SymbolAlreadyDefined,
            })?;

        let ty_id = self.tys.declare_struct(item_id);
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
        fields: Vec<Field>,
    ) -> Result<bool, StructHasInfiniteSize> {
        let ty_id = self.items[item_id].as_ty_id().expect("item is not a type");

        for field in fields.iter() {
            match self.ty_contains_ty(field.ty_id, ty_id) {
                Some(true) => {
                    return Err(StructHasInfiniteSize {
                        path: self.paths[item_id].into(),
                    })
                }
                Some(false) => (),
                None => return Ok(false),
            }
        }

        ty_id.define_struct(fields);
        Ok(true)
    }

    fn ty_contains_ty(&self, haysack_ty_id: TyId, needle_ty_id: TyId) -> Option<bool> {
        let mut tys_to_check = SmallVec::<[TyId; 7]>::new();
        tys_to_check.push(haysack_ty_id);

        while let Some(ty_id) = tys_to_check.pop() {
            if ty_id == needle_ty_id {
                return Some(true);
            }

            if !ty_id.is_defined() {
                return None;
            }

            match ty_id.as_value().unwrap() {
                ValueTy::Array(array_ty) => {
                    tys_to_check.push(array_ty.elem);
                }
                ValueTy::Tuple(tuple_ty) => {
                    tys_to_check.extend(tuple_ty.fields.iter().map(|field| field.ty_id));
                }
                ValueTy::Struct(struct_ty) => {
                    let struct_def = struct_ty.def.lock().unwrap();
                    let struct_fields = &struct_def.as_ref().unwrap().fields;
                    tys_to_check.extend(struct_fields.iter().map(|field| field.ty_id));
                }
                _ => (),
            }
        }

        Some(false)
    }
}
