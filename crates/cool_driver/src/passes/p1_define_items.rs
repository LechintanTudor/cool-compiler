use crate::{CompileResult, ParsedAlias, ParsedCrate, ParsedLiteral};
use cool_ast::{resolve_int_literal, resolve_ty};
use cool_parser::LiteralKind;
use cool_resolve::{tys, ConstItemValue, ResolveContext};
use std::collections::VecDeque;

pub fn p1_define_items(
    mut parsed_crate: ParsedCrate,
    context: &mut ResolveContext,
) -> CompileResult<()> {
    let mut made_progress = true;

    while made_progress {
        made_progress = false;
        made_progress |= define_items(&mut parsed_crate.aliases, context, define_alias);
        made_progress |= define_items(&mut parsed_crate.literals, context, define_literal);
    }

    if !parsed_crate.aliases.is_empty() || !parsed_crate.literals.is_empty() {
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
