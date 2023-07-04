use crate::{
    Alias, CompileError, CompileErrorBundle, CompileResult, DefineError, DefineItem, Package,
    Struct,
};
use cool_ast::AstGenerator;
use cool_resolve::{ResolveContext, TyId};
use smallvec::SmallVec;
use std::collections::VecDeque;

pub fn p2_define_tys(package: &Package, resolve: &mut ResolveContext) -> CompileResult<()> {
    let mut ast = AstGenerator::new(resolve);
    let mut aliases = package.aliases.iter().collect::<VecDeque<_>>();
    let mut structs = package.structs.iter().collect::<VecDeque<_>>();
    let mut ty_ids = VecDeque::<TyId>::new();

    loop {
        let mut made_progress = define_aliases(&mut ast, &mut aliases);
        made_progress |= define_structs(&mut ast, &mut structs);
        made_progress |= define_ty_ids(&mut ast, &mut ty_ids);

        if !made_progress {
            break;
        }
    }

    let mut errors = Vec::<CompileError>::new();
    report_undefinable_items(&mut errors, aliases);
    report_undefinable_items(&mut errors, structs);
    report_undefinable_ty_ids(&mut errors, ty_ids);

    if !errors.is_empty() {
        return Err(CompileErrorBundle { errors });
    }

    Ok(())
}

fn define_aliases(ast: &mut AstGenerator, aliases: &mut VecDeque<&Alias>) -> bool {
    let start_len = aliases.len();

    for _ in 0..start_len {
        let Some(alias_item) = aliases.pop_front() else {
            break;
        };

        let Ok(ty_id) = ast.resolve_ty(alias_item.module_id, &alias_item.item.ty) else {
            aliases.push_back(alias_item);
            continue;
        };

        ast.resolve.define_alias(alias_item.item_id, ty_id);
    }

    aliases.len() < start_len
}

fn define_structs(ast: &mut AstGenerator, structs: &mut VecDeque<&Struct>) -> bool {
    let start_len = structs.len();

    for _ in 0..start_len {
        let Some(struct_item) = structs.pop_front() else {
            break;
        };

        if !struct_item.item.has_body {
            continue;
        }

        let Ok(fields) = struct_item
            .item
            .fields
            .iter()
            .map(|field| {
                ast.resolve_ty(struct_item.module_id, &field.ty)
                    .map(|ty_id| (field.ident.symbol, ty_id))
            })
            .collect::<Result<SmallVec<[_; 7]>, _>>() else {
                structs.push_back(struct_item);
                continue;
            };

        if ast
            .resolve
            .define_struct(struct_item.item_id, fields)
            .is_err()
        {
            structs.push_back(struct_item);
        }
    }

    structs.len() < start_len
}

fn define_ty_ids(ast: &mut AstGenerator, ty_ids: &mut VecDeque<TyId>) -> bool {
    ty_ids.clear();
    ty_ids.extend(ast.resolve.iter_undefined_value_ty_ids());

    let start_len = ty_ids.len();

    for _ in 0..start_len {
        let Some(ty_id) = ty_ids.pop_front() else {
            break;
        };

        if !ast.resolve.define_ty(ty_id) {
            ty_ids.push_back(ty_id);
        }
    }

    ty_ids.len() < start_len
}

fn report_undefinable_items<'a, I>(
    errors: &mut Vec<CompileError>,
    items: impl IntoIterator<Item = &'a DefineItem<I>>,
) where
    I: 'a,
{
    items.into_iter().for_each(|item| {
        errors.push(CompileError::from(DefineError {
            span: Some(item.span),
            kind: item.item_id.into(),
        }));
    })
}

fn report_undefinable_ty_ids(
    errors: &mut Vec<CompileError>,
    ty_ids: impl IntoIterator<Item = TyId>,
) {
    ty_ids.into_iter().for_each(|ty_id| {
        errors.push(CompileError::from(DefineError {
            span: None,
            kind: ty_id.into(),
        }));
    })
}
