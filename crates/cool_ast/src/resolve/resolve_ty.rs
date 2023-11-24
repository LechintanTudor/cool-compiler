use crate::{resolve_int_literal, SpannedAstResult, WithSpan};
use cool_collections::SmallVec;
use cool_parser::{ArrayLen, ItemKind, Ty};
use cool_resolve::{tys, FnAbi, ModuleId, ResolveContext, ResolveError, Scope, TyId};
use cool_span::Section;

pub fn resolve_ty<S>(context: &mut ResolveContext, scope: S, ty: &Ty) -> SpannedAstResult<TyId>
where
    S: Into<Scope>,
{
    resolve_ty_inner(context, context.get_toplevel_module(scope), ty)
}

pub fn resolve_ty_inner(
    context: &mut ResolveContext,
    module_id: ModuleId,
    ty: &Ty,
) -> SpannedAstResult<TyId> {
    let ty_id = match ty {
        Ty::Array(array_ty) => {
            let elem_ty = resolve_ty_inner(context, module_id, &array_ty.elem_ty)?;
            let len = resolve_array_len(context, module_id, &array_ty.len)?;
            context.add_array_ty(elem_ty, len)
        }
        Ty::Fn(fn_ty) => {
            let abi = match &fn_ty.abi_decl {
                Some(abi_decl) => {
                    match abi_decl.abi {
                        Some(abi) => FnAbi::try_from(abi).with_span(abi_decl.span)?,
                        None => FnAbi::C,
                    }
                }
                None => FnAbi::Cool,
            };

            let param_tys = fn_ty
                .params
                .param_tys
                .iter()
                .map(|ty| resolve_ty_inner(context, module_id, ty))
                .collect::<Result<SmallVec<_, 12>, _>>()?;

            let return_ty = fn_ty
                .return_ty
                .as_ref()
                .map(|ty| resolve_ty_inner(context, module_id, ty))
                .transpose()?
                .unwrap_or(tys::unit);

            context.add_fn_ty(abi, param_tys, fn_ty.params.is_variadic, return_ty)
        }
        Ty::Item(item_ty) => {
            match item_ty.kind {
                ItemKind::Alias => tys::alias,
                ItemKind::Module => tys::module,
            }
        }
        Ty::ManyPtr(many_ptr_ty) => {
            let pointee_ty = resolve_ty_inner(context, module_id, &many_ptr_ty.pointee_ty)?;
            context.add_many_ptr_ty(pointee_ty, many_ptr_ty.is_mutable)
        }
        Ty::Paren(paren_ty) => resolve_ty_inner(context, module_id, &paren_ty.ty)?,
        Ty::Path(ident_path) => {
            let path = ident_path
                .idents
                .iter()
                .map(|ident| ident.symbol)
                .collect::<SmallVec<_, 8>>();

            let item_id = context
                .resolve_path(module_id, &path)
                .with_span(ident_path.span())?;

            context[item_id]
                .try_as_ty()
                .filter(TyId::is_definable)
                .ok_or(ResolveError::ItemNotTy { item_id })
                .with_span(ident_path.span())?
        }
        Ty::Ptr(ptr_ty) => {
            let pointee_ty = resolve_ty_inner(context, module_id, &ptr_ty.pointee_ty)?;
            context.add_ptr_ty(pointee_ty, ptr_ty.is_mutable)
        }
        Ty::Slice(slice_ty) => {
            let elem_ty = resolve_ty_inner(context, module_id, &slice_ty.elem_ty)?;
            context.add_slice_ty(elem_ty, slice_ty.is_mutable)
        }
        Ty::Tuple(tuple_ty) => {
            let elem_tys = tuple_ty
                .elem_tys
                .iter()
                .map(|ty| resolve_ty_inner(context, module_id, ty))
                .collect::<Result<SmallVec<_, 8>, _>>()?;

            context.add_tuple_ty(elem_tys)
        }
        Ty::Variant(variant_ty) => {
            let variant_tys = variant_ty
                .variant_tys
                .iter()
                .map(|ty| resolve_ty_inner(context, module_id, ty))
                .collect::<Result<SmallVec<_, 8>, _>>()?;

            context.add_variant_ty(variant_tys)
        }
    };

    Ok(ty_id)
}

fn resolve_array_len(
    context: &ResolveContext,
    module_id: ModuleId,
    len: &ArrayLen,
) -> SpannedAstResult<u64> {
    let value = match len {
        ArrayLen::Int(value) => {
            resolve_int_literal(value.value.as_str())
                .map(|(value, _)| value as u64)
                .with_span(value.span)?
        }
        ArrayLen::Path(ident_path) => {
            let path = ident_path
                .idents
                .iter()
                .map(|ident| ident.symbol)
                .collect::<SmallVec<_, 8>>();

            let item_id = context
                .resolve_path(module_id, &path)
                .with_span(ident_path.span())?;

            let const_id = context[item_id]
                .try_as_const()
                .ok_or(ResolveError::ItemNotConst { item_id })
                .with_span(ident_path.span())?;

            context[const_id]
                .value
                .try_as_int()
                .map(|value| value as u64)
                .ok_or(ResolveError::ItemNotUsize { item_id })
                .with_span(ident_path.span())?
        }
    };

    Ok(value)
}
