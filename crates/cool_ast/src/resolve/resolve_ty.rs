use crate::AstResult;
use cool_parser::{ItemKind, Ty};
use cool_resolve::{tys, FnAbi, ModuleId, ResolveContext, ResolveError, Scope, TyId};
use smallvec::SmallVec;

pub fn resolve_ty<S>(context: &mut ResolveContext, scope: S, ty: &Ty) -> AstResult<TyId>
where
    S: Into<Scope>,
{
    resolve_ty_inner(context, context.get_toplevel_module(scope), ty)
}

pub fn resolve_ty_inner(
    context: &mut ResolveContext,
    module_id: ModuleId,
    ty: &Ty,
) -> AstResult<TyId> {
    let ty_id = match ty {
        Ty::Array(array_ty) => {
            todo!("{:#?}", array_ty);
        }
        Ty::Fn(fn_ty) => {
            let abi = match &fn_ty.abi_decl {
                Some(abi_decl) => {
                    match abi_decl.abi {
                        Some(abi) => FnAbi::try_from(abi)?,
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
                .collect::<Result<SmallVec<[_; 12]>, _>>()?;

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
        Ty::Path(path) => {
            let path = path
                .idents
                .iter()
                .map(|ident| ident.symbol)
                .collect::<SmallVec<[_; 8]>>();

            let item_id = context.resolve_path(module_id, &path)?;

            context[item_id]
                .try_as_ty()
                .filter(TyId::is_defined)
                .ok_or(ResolveError::ItemNotTy { item_id })?
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
                .collect::<Result<SmallVec<[_; 8]>, _>>()?;

            context.add_tuple_ty(elem_tys)
        }
    };

    Ok(ty_id)
}
