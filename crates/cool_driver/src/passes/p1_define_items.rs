use crate::{
    CompileResult, DefinedCrate, ParsedAlias, ParsedCrate, ParsedExternFn, ParsedFn, ParsedLiteral,
    ParsedStruct,
};
use cool_ast::{resolve_fn, resolve_int_literal, resolve_ty};
use cool_collections::SmallVec;
use cool_parser::LiteralKind;
use cool_resolve::{tys, ConstItemValue, ResolveContext, TyId};
use std::collections::VecDeque;

pub fn p1_define_items(
    mut parsed_crate: ParsedCrate,
    context: &mut ResolveContext,
) -> CompileResult<DefinedCrate> {
    let mut undefined_tys = Vec::new();
    let fns = parsed_crate.fns.iter().cloned().collect::<Vec<_>>();

    loop {
        let mut made_progress = false;
        made_progress |= define_items(&mut parsed_crate.aliases, context, define_alias);
        made_progress |= define_items(&mut parsed_crate.literals, context, define_literal);
        made_progress |= define_items(&mut parsed_crate.structs, context, define_struct);

        undefined_tys.clear();
        undefined_tys.extend(context.iter_undefined_ty_ids());
        made_progress |= define_tys(&undefined_tys, context);

        if !made_progress {
            break;
        }
    }

    loop {
        let mut made_progress = false;
        made_progress |= define_items(&mut parsed_crate.extern_fns, context, define_extern_fn);
        made_progress |= define_items(&mut parsed_crate.fns, context, define_fn);

        if !made_progress {
            break;
        }
    }

    assert!(parsed_crate.aliases.is_empty());
    assert!(parsed_crate.literals.is_empty());
    assert!(parsed_crate.extern_fns.is_empty());
    assert!(parsed_crate.fns.is_empty());

    Ok(DefinedCrate {
        files: parsed_crate.files,
        fns,
    })
}

fn define_items<I, F>(
    items: &mut VecDeque<I>,
    context: &mut ResolveContext,
    mut define_item: F,
) -> bool
where
    F: FnMut(&I, &mut ResolveContext) -> CompileResult<()>,
{
    let mut made_progress = false;
    let items_len = items.len();

    for _ in 0..items_len {
        let Some(item) = items.pop_front() else {
            break;
        };

        if define_item(&item, context).is_ok() {
            made_progress = true;
        } else {
            items.push_back(item);
        }
    }

    made_progress
}

fn define_tys(ty_ids: &[TyId], context: &mut ResolveContext) -> bool {
    let mut made_progress = false;

    for &ty_id in ty_ids {
        if context.define_ty(ty_id).is_ok() {
            made_progress = true;
        }
    }

    made_progress
}

fn define_alias(alias: &ParsedAlias, context: &mut ResolveContext) -> CompileResult<()> {
    let item_ty = alias
        .ty
        .as_ref()
        .map(|ty| resolve_ty(context, alias.module_id, ty))
        .transpose()?
        .unwrap_or(tys::alias);

    if item_ty != tys::alias {
        panic!("Invalid item type");
    }

    let ty_id = resolve_ty(context, alias.module_id, &alias.item.ty)?;

    context.update_alias(alias.item_id, ty_id);
    Ok(())
}

fn define_literal(literal: &ParsedLiteral, context: &mut ResolveContext) -> CompileResult<()> {
    let _item_ty = literal
        .ty
        .as_ref()
        .map(|ty| resolve_ty(context, literal.module_id, ty))
        .transpose()?
        .unwrap_or(tys::infer);

    let (value, ty_id) = match literal.item.kind {
        LiteralKind::Int => resolve_int_literal(literal.item.value.as_str())?,
        _ => todo!(),
    };

    context.update_const(literal.item_id, ty_id, ConstItemValue::Int(value));
    Ok(())
}

fn define_struct(struct_item: &ParsedStruct, context: &mut ResolveContext) -> CompileResult<()> {
    let ty_id = context[struct_item.item_id].try_as_ty().unwrap();

    let fields = struct_item
        .item
        .fields
        .iter()
        .map(|field| {
            resolve_ty(context, struct_item.module_id, &field.ty)
                .map(|ty_id| (field.ident.symbol, ty_id))
        })
        .collect::<Result<SmallVec<_, 8>, _>>()?;

    context.define_struct_ty(ty_id, fields)?;
    Ok(())
}

fn define_extern_fn(fn_item: &ParsedExternFn, context: &mut ResolveContext) -> CompileResult<()> {
    let expected_ty_id = fn_item
        .ty
        .as_deref()
        .map(|ty| resolve_ty(context, fn_item.module_id, ty))
        .transpose()?;

    let ty_id = resolve_fn(
        context,
        fn_item.module_id,
        expected_ty_id,
        &fn_item.item.prototype,
    )?;

    context.update_const(fn_item.item_id, ty_id, ConstItemValue::Fn);
    Ok(())
}

fn define_fn(fn_item: &ParsedFn, context: &mut ResolveContext) -> CompileResult<()> {
    let expected_ty_id = fn_item
        .ty
        .as_deref()
        .map(|ty| resolve_ty(context, fn_item.module_id, ty))
        .transpose()?;

    let ty_id = resolve_fn(
        context,
        fn_item.module_id,
        expected_ty_id,
        &fn_item.item.prototype,
    )?;

    context.update_const(fn_item.item_id, ty_id, ConstItemValue::Fn);
    Ok(())
}
