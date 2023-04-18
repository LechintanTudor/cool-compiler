use crate::Package;
use cool_lexer::symbols::sym;
use cool_parser::{AliasItem, Item, ModuleContent, ModuleItem, ModuleKind, StructItem, Ty};
use cool_resolve::{
    tys, ItemId, ItemPathBuf, ModuleId, ResolveContext, ResolveError, ResolveResult, ScopeId,
    StructTy, TyId,
};
use smallvec::SmallVec;
use std::collections::VecDeque;

pub fn p1_define_tys(package: &Package, resolve: &mut ResolveContext) {
    let mut aliases = VecDeque::<(ModuleId, ItemId, &AliasItem)>::new();
    let mut structs = VecDeque::<(ModuleId, ItemId, &StructItem)>::new();

    for source in package.sources.iter() {
        let mut modules = VecDeque::<(ModuleId, &ModuleContent)>::new();
        modules.push_back((source.module_id, &source.module));

        while let Some((module_id, module)) = modules.pop_front() {
            for item_decl in module
                .decls
                .iter()
                .filter_map(|decl| decl.kind.as_item_decl())
            {
                match &item_decl.item {
                    Item::Alias(alias_item) => {
                        aliases.push_back((module_id, item_decl.item_id, alias_item));
                    }
                    Item::Struct(struct_item) => {
                        structs.push_back((module_id, item_decl.item_id, struct_item));
                    }
                    Item::Module(ModuleItem {
                        kind: ModuleKind::Inline(module_content),
                        ..
                    }) => {
                        modules.push_back((module_id, module_content));
                    }
                    _ => (),
                }
            }
        }
    }

    let mut resolve_fail_count = 0;
    while let Some((module_id, item_id, alias)) = aliases.pop_front() {
        match resolve_parsed_ty(resolve, module_id.into(), &alias.ty) {
            Ok(resolved_ty_id) => {
                resolve.define_alias(item_id, resolved_ty_id);
                resolve_fail_count = 0;
            }
            Err(_) => {
                aliases.push_back((module_id, item_id, alias));

                resolve_fail_count += 1;
                if resolve_fail_count >= aliases.len() {
                    panic!("failed to resolve type aliases");
                }
            }
        }
    }

    let mut resolve_fail_count = 0;
    while let Some((module_id, item_id, struct_item)) = structs.pop_front() {
        let struct_ty = {
            let mut struct_ty = StructTy::default();

            for field in struct_item.fields.iter() {
                struct_ty.fields.insert_if_not_exists(
                    field.ident.symbol,
                    resolve_parsed_ty(resolve, module_id.into(), &field.ty).unwrap(),
                );
            }

            struct_ty
        };

        match resolve.define_struct(item_id, struct_ty) {
            Some(true) => resolve_fail_count = 0,
            Some(false) => {
                structs.push_back((module_id, item_id, struct_item));

                resolve_fail_count += 1;
                if resolve_fail_count >= structs.len() {
                    panic!("failed to resolve structs");
                }
            }
            _ => todo!(),
        }
    }
}

pub fn resolve_parsed_ty(
    resolve: &mut ResolveContext,
    scope_id: ScopeId,
    parsed_ty: &Ty,
) -> ResolveResult<TyId> {
    match parsed_ty {
        Ty::Fn(fn_ty) => {
            let mut param_ty_ids = SmallVec::<[TyId; 6]>::new();

            let abi = fn_ty
                .extern_decl
                .as_ref()
                .map(|decl| decl.abi.unwrap_or(sym::ABI_C))
                .unwrap_or(sym::ABI_COOL);

            for param in fn_ty.param_list.params.iter() {
                param_ty_ids.push(resolve_parsed_ty(resolve, scope_id, param)?);
            }

            let ret_ty_id = match &fn_ty.ret_ty {
                Some(ret_ty) => resolve_parsed_ty(resolve, scope_id, ret_ty)?,
                None => tys::UNIT,
            };

            Ok(resolve.mk_fn(abi, param_ty_ids, ret_ty_id))
        }
        Ty::Path(path) => {
            let path = path
                .idents
                .iter()
                .map(|ident| ident.symbol)
                .collect::<ItemPathBuf>();

            let item_id = resolve.resolve_global(scope_id, &path)?;

            resolve[item_id]
                .as_ty_id()
                .filter(|ty| !ty.is_inferred())
                .ok_or(ResolveError::not_ty(path.last()))
        }
        Ty::Tuple(tuple_ty) => {
            let mut elem_tys = SmallVec::<[TyId; 6]>::new();

            for ty in tuple_ty.elems.iter() {
                elem_tys.push(resolve_parsed_ty(resolve, scope_id, ty)?);
            }

            Ok(resolve.mk_tuple(elem_tys))
        }
        _ => todo!(),
    }
}
