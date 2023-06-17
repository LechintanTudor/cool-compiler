use crate::{CompileError, CompileErrorBundle, CompileResult, Package};
use cool_ast::AstGenerator;
use cool_resolve::{
    DefineError, Field, ResolveContext, StructHasDuplicatedField, TyCannotBeDefined,
};
use std::collections::VecDeque;

pub fn p2_define_tys(package: &Package, resolve: &mut ResolveContext) -> CompileResult<()> {
    let mut ast = AstGenerator::new(resolve);
    let mut errors = Vec::<CompileError>::new();

    let mut aliases = package
        .aliases
        .iter()
        .map(|alias| (alias.module_id, alias.item_id, &alias.item))
        .collect::<VecDeque<_>>();

    let mut resolve_fail_count = 0;
    while let Some((module_id, item_id, alias)) = aliases.pop_front() {
        match ast.resolve_ty(module_id.into(), &alias.ty) {
            Ok(resolved_ty_id) => {
                ast.resolve.define_alias(item_id, resolved_ty_id);
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

    let mut structs = package
        .structs
        .iter()
        .map(|struct_item| {
            (
                struct_item.module_id,
                struct_item.item_id,
                &struct_item.item,
            )
        })
        .collect::<VecDeque<_>>();

    let mut resolve_fail_count = 0;
    'struct_loop: while let Some((module_id, item_id, struct_item)) = structs.pop_front() {
        let fields = {
            let mut fields = Vec::<Field>::new();

            for field in struct_item.fields.iter() {
                let ty_id = match ast.resolve_ty(module_id.into(), &field.ty) {
                    Ok(ty_id) => ty_id,
                    Err(error) => {
                        errors.push(error.into());
                        continue 'struct_loop;
                    }
                };

                let is_duplicated = fields
                    .iter()
                    .any(|ty_field| ty_field.symbol == field.ident.symbol);

                if is_duplicated {
                    errors.push(
                        DefineError::from(StructHasDuplicatedField {
                            path: ast.resolve.get_path_by_item_id(item_id).to_path_buf(),
                            field: field.ident.symbol,
                        })
                        .into(),
                    );
                    continue 'struct_loop;
                }

                fields.push(Field {
                    offset: 0,
                    symbol: field.ident.symbol,
                    ty_id,
                });
            }

            fields
        };

        match ast.resolve.define_struct(item_id, fields) {
            Ok(true) => resolve_fail_count = 0,
            Ok(false) => {
                structs.push_back((module_id, item_id, struct_item));

                resolve_fail_count += 1;
                if resolve_fail_count >= structs.len() {
                    break;
                }
            }
            Err(error) => {
                errors.push(DefineError::from(error).into());
            }
        }
    }

    while let Some((_, item_id, _)) = aliases.pop_front() {
        errors.push(
            DefineError::from(TyCannotBeDefined {
                path: ast.resolve.get_path_by_item_id(item_id).to_path_buf(),
            })
            .into(),
        );
    }

    while let Some((_, item_id, _)) = structs.pop_front() {
        errors.push(
            DefineError::from(TyCannotBeDefined {
                path: ast.resolve.get_path_by_item_id(item_id).to_path_buf(),
            })
            .into(),
        );
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(CompileErrorBundle { errors })
    }
}
