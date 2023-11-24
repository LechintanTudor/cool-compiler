use crate::{
    DefineError, DefinedCrate, ParsedAlias, ParsedCrate, ParsedExternFn, ParsedFn, ParsedItem,
    ParsedLiteral, ParsedStruct, SpannedCompileError, SpannedCompileResult, WithLocation,
    WithSourceId,
};
use cool_ast::{resolve_fn, resolve_int_literal, resolve_ty};
use cool_collections::SmallVec;
use cool_parser::LiteralKind;
use cool_resolve::{tys, ConstItemValue, ResolveContext, TyId};
use std::collections::VecDeque;

pub fn p1_define_items(
    parsed_crate: ParsedCrate,
    context: &mut ResolveContext,
    errors: &mut Vec<SpannedCompileError>,
) -> DefinedCrate {
    let mut aliases = VecDeque::from(parsed_crate.aliases);
    let mut literals = VecDeque::from(parsed_crate.literals);
    let mut structs = VecDeque::from(parsed_crate.structs);
    let mut undefined_tys = Vec::new();

    loop {
        let mut made_progress = false;
        made_progress |= define_items(&mut aliases, context, define_alias);
        made_progress |= define_items(&mut literals, context, define_literal);
        made_progress |= define_items(&mut structs, context, define_struct);

        undefined_tys.clear();
        undefined_tys.extend(context.iter_undefined_ty_ids());
        made_progress |= define_tys(&undefined_tys, context);

        if !made_progress {
            break;
        }
    }

    let mut extern_fns = VecDeque::from(parsed_crate.extern_fns);
    let mut fns = VecDeque::from(parsed_crate.fns.clone());

    loop {
        let mut made_progress = false;
        made_progress |= define_items(&mut extern_fns, context, define_extern_fn);
        made_progress |= define_items(&mut fns, context, define_fn);

        if !made_progress {
            break;
        }
    }

    report_undefinable_items(errors, aliases);
    report_undefinable_items(errors, literals);
    report_undefinable_items(errors, structs);
    report_undefinable_items(errors, extern_fns);
    report_undefinable_items(errors, fns);

    DefinedCrate {
        files: parsed_crate.files,
        fns: parsed_crate.fns,
    }
}

fn define_items<I, F>(
    items: &mut VecDeque<I>,
    context: &mut ResolveContext,
    mut define_item: F,
) -> bool
where
    F: FnMut(&I, &mut ResolveContext) -> SpannedCompileResult<()>,
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

fn define_alias(alias: &ParsedAlias, context: &mut ResolveContext) -> SpannedCompileResult<()> {
    let item_ty = alias
        .ty
        .as_ref()
        .map(|ty| resolve_ty(context, alias.module_id, ty))
        .transpose()
        .with_source_id(alias.source_id)?
        .unwrap_or(tys::alias);

    if item_ty != tys::alias {
        panic!("Invalid item type");
    }

    let ty_id =
        resolve_ty(context, alias.module_id, &alias.item.ty).with_source_id(alias.source_id)?;

    context.update_alias(alias.item_id, ty_id);
    Ok(())
}

fn define_literal(
    literal: &ParsedLiteral,
    context: &mut ResolveContext,
) -> SpannedCompileResult<()> {
    let _item_ty = literal
        .ty
        .as_ref()
        .map(|ty| resolve_ty(context, literal.module_id, ty))
        .transpose()
        .with_source_id(literal.source_id)?
        .unwrap_or(tys::infer);

    let (value, ty_id) = match literal.item.kind {
        LiteralKind::Int => {
            resolve_int_literal(literal.item.value.as_str())
                .with_location((literal.source_id, literal.span))?
        }
        _ => todo!(),
    };

    context.update_const(literal.item_id, ty_id, ConstItemValue::Int(value));
    Ok(())
}

fn define_struct(
    struct_item: &ParsedStruct,
    context: &mut ResolveContext,
) -> SpannedCompileResult<()> {
    let ty_id = context[struct_item.item_id].try_as_ty().unwrap();

    let fields = struct_item
        .item
        .fields
        .iter()
        .map(|field| {
            resolve_ty(context, struct_item.module_id, &field.ty)
                .map(|ty_id| (field.ident.symbol, ty_id))
        })
        .collect::<Result<SmallVec<_, 8>, _>>()
        .with_source_id(struct_item.source_id)?;

    context
        .define_struct_ty(ty_id, fields)
        .with_location((struct_item.source_id, struct_item.span))?;

    Ok(())
}

fn define_extern_fn(
    fn_item: &ParsedExternFn,
    context: &mut ResolveContext,
) -> SpannedCompileResult<()> {
    let expected_ty_id = fn_item
        .ty
        .as_deref()
        .map(|ty| resolve_ty(context, fn_item.module_id, ty))
        .transpose()
        .with_source_id(fn_item.source_id)?;

    let ty_id = resolve_fn(
        context,
        fn_item.module_id,
        expected_ty_id,
        &fn_item.item.prototype,
    )
    .with_source_id(fn_item.source_id)?;

    context.update_const(fn_item.item_id, ty_id, ConstItemValue::Fn);
    Ok(())
}

fn define_fn(fn_item: &ParsedFn, context: &mut ResolveContext) -> SpannedCompileResult<()> {
    let expected_ty_id = fn_item
        .ty
        .as_deref()
        .map(|ty| resolve_ty(context, fn_item.module_id, ty))
        .transpose()
        .with_source_id(fn_item.source_id)?;

    let ty_id = resolve_fn(
        context,
        fn_item.module_id,
        expected_ty_id,
        &fn_item.item.prototype,
    )
    .with_source_id(fn_item.source_id)?;

    context.update_const(fn_item.item_id, ty_id, ConstItemValue::Fn);
    Ok(())
}

fn report_undefinable_items<I>(
    errors: &mut Vec<SpannedCompileError>,
    items: VecDeque<ParsedItem<I>>,
) {
    for item in items {
        errors.push(SpannedCompileError::new(
            (item.source_id, item.span),
            DefineError {
                item_id: item.item_id,
            },
        ));
    }
}
