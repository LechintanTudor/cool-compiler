use crate::{CompileResult, ParsedAlias, ParsedCrate, ParsedLiteral, ParsedStruct};
use cool_ast::{resolve_int_literal, resolve_ty};
use cool_parser::LiteralKind;
use cool_resolve::{tys, ConstItemValue, ResolveContext, TyId};
use smallvec::SmallVec;
use std::collections::VecDeque;

pub fn p1_define_items(
    mut parsed_crate: ParsedCrate,
    context: &mut ResolveContext,
) -> CompileResult<()> {
    let mut made_progress = true;
    let mut undefined_tys = Vec::new();

    while made_progress {
        undefined_tys.clear();
        undefined_tys.extend(context.iter_undefined_ty_ids());

        made_progress = false;
        made_progress |= define_items(&mut parsed_crate.aliases, context, define_alias);
        made_progress |= define_items(&mut parsed_crate.literals, context, define_literal);
        made_progress |= define_items(&mut parsed_crate.structs, context, define_struct);
        made_progress |= define_tys(&undefined_tys, context);
    }

    let is_fully_defined = parsed_crate.aliases.is_empty()
        && parsed_crate.literals.is_empty()
        && parsed_crate.structs.is_empty()
        && undefined_tys.is_empty();

    if !is_fully_defined {
        panic!("Failed to define items");
    }

    Ok(())
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

    let (value, _ty_id) = match literal.item.kind {
        LiteralKind::Int => resolve_int_literal(literal.item.value.as_str())?,
        _ => todo!(),
    };

    context.update_const(literal.item_id, tys::usize, ConstItemValue::Int(value));
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
        .collect::<Result<SmallVec<[_; 8]>, _>>()?;

    context.define_aggregate_ty(ty_id, &fields)?;
    Ok(())
}