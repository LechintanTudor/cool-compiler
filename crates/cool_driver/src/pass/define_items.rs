use crate::pass::{Item, Project};
use cool_ir::resolve_ty;
use cool_resolve::{tys, ResolveContext, ResolveResult};
use std::collections::VecDeque;
use std::mem;

pub fn define_items(project: &mut Project, context: &mut ResolveContext) {
    let mut items = VecDeque::<Item>::from(mem::take(&mut project.items));

    loop {
        let mut made_progress = false;

        for _ in 0..items.len() {
            let item = items.pop_front().unwrap();

            match define_item(project, context, &item) {
                Ok(()) => made_progress = true,
                Err(_) => items.push_back(item),
            }
        }

        if !made_progress {
            break;
        }
    }

    project.items = items.into();
}

fn define_item(project: &Project, context: &mut ResolveContext, item: &Item) -> ResolveResult {
    let ast_file = &project.files[item.ast_file_id].parsed_file;
    let ast_item = &ast_file.items[item.ast_item_id];

    let _item_ty = ast_item
        .ty
        .map(|ast_ty_id| resolve_ty(context, item.module_id, ast_file, ast_ty_id))
        .transpose()?
        .unwrap_or(tys::infer);

    match ast_item.kind {
        ast::ItemKind::Alias(ast_ty_id) => {
            let ty = resolve_ty(context, item.module_id, ast_file, ast_ty_id)?;
            context.define_alias(item.item_id, ty);
        }
        _ => todo!(),
    }

    Ok(())
}
