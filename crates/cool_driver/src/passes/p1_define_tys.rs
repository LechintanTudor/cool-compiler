use crate::{CompileError, CompileErrorBundle, CompileErrorKind, CompileResult, Package};
use cool_parser::{AliasItem, Item, ModuleContent, ModuleItem, ModuleKind, StructItem};
use cool_resolve::{
    ItemId, ModuleId, ResolveContext, StructHasDuplicatedField, StructTy, TypeCannotBeDefined,
};
use std::collections::VecDeque;

pub fn p1_define_tys(package: &Package, resolve: &mut ResolveContext) -> CompileResult<()> {
    let mut errors = Vec::<CompileError>::new();
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
        match cool_ast::resolve_ty(resolve, module_id.into(), &alias.ty) {
            Ok(resolved_ty_id) => {
                resolve.define_alias(item_id, resolved_ty_id);
                resolve_fail_count = 0;
            }
            Err(_) => {
                aliases.push_back((module_id, item_id, alias));

                resolve_fail_count += 1;
                if resolve_fail_count >= aliases.len() {
                    break;
                }
            }
        }
    }

    let mut resolve_fail_count = 0;
    'struct_loop: while let Some((module_id, item_id, struct_item)) = structs.pop_front() {
        let struct_ty = {
            let mut struct_ty = StructTy::default();

            for field in struct_item.fields.iter() {
                let ty_id = match cool_ast::resolve_ty(resolve, module_id.into(), &field.ty) {
                    Ok(ty_id) => ty_id,
                    Err(error) => {
                        errors.push(CompileError {
                            path: Default::default(),
                            kind: error.into(),
                        });
                        continue 'struct_loop;
                    }
                };

                let inserted_successfully = struct_ty
                    .fields
                    .insert_if_not_exists(field.ident.symbol, ty_id);

                if !inserted_successfully {
                    errors.push(CompileError {
                        path: Default::default(),
                        kind: CompileErrorKind::Define(
                            StructHasDuplicatedField {
                                path: resolve.get_path_by_item_id(item_id).to_path_buf(),
                                field: field.ident.symbol,
                            }
                            .into(),
                        ),
                    });
                    continue 'struct_loop;
                }
            }

            struct_ty
        };

        match resolve.define_struct(item_id, struct_ty) {
            Ok(true) => resolve_fail_count = 0,
            Ok(false) => {
                structs.push_back((module_id, item_id, struct_item));

                resolve_fail_count += 1;
                if resolve_fail_count >= structs.len() {
                    break;
                }
            }
            Err(error) => {
                errors.push(CompileError {
                    path: Default::default(),
                    kind: CompileErrorKind::Define(error.into()),
                });
            }
        }
    }

    while let Some((_, item_id, _)) = aliases.pop_front() {
        errors.push(CompileError {
            path: Default::default(),
            kind: CompileErrorKind::Define(
                TypeCannotBeDefined {
                    path: resolve.get_path_by_item_id(item_id).to_path_buf(),
                }
                .into(),
            ),
        });
    }

    while let Some((_, item_id, _)) = structs.pop_front() {
        errors.push(CompileError {
            path: Default::default(),
            kind: CompileErrorKind::Define(
                TypeCannotBeDefined {
                    path: resolve.get_path_by_item_id(item_id).to_path_buf(),
                }
                .into(),
            ),
        });
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(CompileErrorBundle { errors })
    }
}
