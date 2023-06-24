use crate::{
    DefineError, DefineErrorKind, DefineResult, ItemId, ItemKind, ModuleElem, ModuleId,
    ResolveContext, ResolveError, ResolveErrorKind, ResolveResult, StructTy, TyId, ValueTy,
};
use cool_lexer::Symbol;
use rustc_hash::FxHashSet;
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
            .insert_slice_if_not_exists(item_path.as_symbol_slice())
            .ok_or(ResolveError {
                symbol,
                kind: ResolveErrorKind::SymbolAlreadyDefined,
            })?;

        let ty_id = self.tys.insert_value(StructTy { item_id });
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

    pub fn define_struct(
        &mut self,
        item_id: ItemId,
        fields: Vec<(Symbol, TyId)>,
    ) -> DefineResult<bool> {
        let ty_id = self.items[item_id]
            .as_ty_id()
            .expect("item is not a struct");

        let mut field_names = FxHashSet::<Symbol>::default();

        for (field_symbol, field_ty_id) in fields.iter().copied() {
            if !field_names.insert(field_symbol) {
                return Err(DefineError {
                    path: self.paths[item_id].into(),
                    kind: DefineErrorKind::StructHasDuplicatedField {
                        field: field_symbol,
                    },
                });
            }

            match self.ty_contains_ty(field_ty_id, ty_id) {
                Some(true) => {
                    return Err(DefineError {
                        path: self.paths[item_id].into(),
                        kind: DefineErrorKind::StructHasInfiniteSize,
                    });
                }
                Some(false) => (),
                None => return Ok(false),
            }
        }

        let _ = ty_id.define_struct(fields);
        Ok(true)
    }

    fn ty_contains_ty(&self, haysack_ty_id: TyId, needle_ty_id: TyId) -> Option<bool> {
        let mut tys_to_check = SmallVec::<[TyId; 7]>::new();
        tys_to_check.push(haysack_ty_id);

        while let Some(ty_id) = tys_to_check.pop() {
            if ty_id == needle_ty_id {
                return Some(true);
            }

            if !ty_id.def.is_defined() {
                return None;
            }

            match ty_id.shape.as_value().unwrap() {
                ValueTy::Array(array_ty) => {
                    tys_to_check.push(array_ty.elem);
                }
                ValueTy::Tuple(tuple_ty) => {
                    tys_to_check.extend(tuple_ty.elems.iter().cloned());
                }
                ValueTy::Struct(_) => {
                    let fields = ty_id.def.get_aggregate_fields()?;
                    let fields = fields.iter().map(|field| field.ty_id);
                    tys_to_check.extend(fields);
                }
                _ => (),
            }
        }

        Some(false)
    }
}
