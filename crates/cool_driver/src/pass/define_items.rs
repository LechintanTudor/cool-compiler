use crate::pass::{Item, Project};
use cool_collections::ahash::AHashSet;
use cool_collections::SmallVec;
use cool_ir::{resolve_fn, resolve_ty};
use cool_lexer::Symbol;
use cool_resolve::{tys, ResolveContext, ResolveError, ResolveResult, TyId};
use std::collections::VecDeque;
use std::mem;

pub fn define_items(project: &mut Project, context: &mut ResolveContext) -> Vec<Item> {
    let mut items = VecDeque::<Item>::from(mem::take(&mut project.items));
    let mut fn_items = Vec::<Item>::new();
    let mut ty_ids = VecDeque::<TyId>::new();

    loop {
        let mut made_progress = false;

        for _ in 0..items.len() {
            let item = items.pop_front().unwrap();

            match define_item(project, context, &item) {
                Ok(is_fn) => {
                    made_progress = true;

                    if is_fn {
                        fn_items.push(item);
                    }
                }
                Err(_) => items.push_back(item),
            }
        }

        ty_ids.clear();
        ty_ids.extend(context.iter_tys_to_be_defined());

        for _ in 0..ty_ids.len() {
            let ty_id = ty_ids.pop_front().unwrap();

            match context.define_ty(ty_id) {
                Some(_) => made_progress = true,
                None => ty_ids.push_back(ty_id),
            }
        }

        if !made_progress {
            break;
        }
    }

    assert!(ty_ids.is_empty());
    project.items = items.into();
    fn_items
}

fn define_item(
    project: &Project,
    context: &mut ResolveContext,
    item: &Item,
) -> ResolveResult<bool> {
    let ast_file = &project.files[item.ast_file_id].parsed_file;
    let ast_item = &ast_file.items[item.ast_item_id];

    let item_ty_id = ast_item
        .ty
        .map(|ast_ty_id| resolve_ty(context, item.module_id, ast_file, ast_ty_id))
        .transpose()?
        .unwrap_or(tys::infer);

    let mut is_fn = false;

    match ast_item.kind {
        ast::ItemKind::Alias(ast_ty_id) => {
            let ty = resolve_ty(context, item.module_id, ast_file, ast_ty_id)?;
            context.define_alias(item.item_id, ty);
        }
        ast::ItemKind::Expr(ast_expr_id) => {
            let ty_id = match &ast_file[ast_expr_id] {
                ast::Expr::Fn(fn_expr) => {
                    is_fn = true;
                    resolve_fn(context, item.module_id, item_ty_id, ast_file, fn_expr.proto)?
                }
                _ => todo!(),
            };

            let binding_id = context[item.item_id].into_binding();
            context.define_global_binding(binding_id, ty_id);
        }
        ast::ItemKind::Struct(ast_struct_id) => {
            let mut fields = SmallVec::<(Symbol, TyId), 8>::new();
            let mut field_names = AHashSet::<Symbol>::new();

            for field in &ast_file[ast_struct_id].fields {
                if !field_names.insert(field.ident.symbol) {
                    return Err(ResolveError::StructHasDuplicatedField {
                        field: field.ident.symbol,
                    });
                }

                let field_ty = resolve_ty(context, item.module_id, ast_file, field.ty)?;
                fields.push((field.ident.symbol, field_ty));
            }

            let ty_id = context[item.item_id].into_ty();
            context.define_struct_ty(ty_id, &fields);
        }
        ast::ItemKind::ExternFn(fn_proto_id) => {
            let ty_id = resolve_fn(context, item.module_id, item_ty_id, ast_file, fn_proto_id)?;
            let binding_id = context[item.item_id].into_binding();
            context.define_global_binding(binding_id, ty_id);
        }
        _ => todo!(),
    }

    Ok(is_fn)
}
