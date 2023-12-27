use cool_ast as ast;
use cool_collections::SmallVec;
use cool_lexer::sym;
use cool_resolve::{tys, FnAbi, Item, ModuleId, ResolveContext, ResolveError, ResolveResult, TyId};

pub fn resolve_ty(
    context: &mut ResolveContext,
    module_id: ModuleId,
    ast_file: &ast::File,
    ast_ty_id: ast::TyId,
) -> ResolveResult<TyId> {
    let ty_id = match &ast_file[ast_ty_id] {
        ast::Ty::Array(_) => todo!(),
        ast::Ty::Fn(fn_ty) => {
            let abi = match fn_ty.abi {
                ast::FnAbi::Implicit => FnAbi::Cool,
                ast::FnAbi::Explicit(None) => FnAbi::C,
                ast::FnAbi::Explicit(Some(sym::C)) => FnAbi::C,
                ast::FnAbi::Explicit(Some(sym::Cool)) => FnAbi::Cool,
                ast::FnAbi::Explicit(Some(abi)) => {
                    return Err(ResolveError::FnAbiIsUnknown { abi });
                }
            };

            let param_tys = fn_ty
                .param_tys
                .iter()
                .map(|&ty_id| resolve_ty(context, module_id, ast_file, ty_id))
                .collect::<Result<SmallVec<_, 8>, _>>()?;

            let return_ty = fn_ty
                .return_ty
                .map(|ty_id| resolve_ty(context, module_id, ast_file, ty_id))
                .transpose()?
                .unwrap_or(tys::unit);

            context.add_fn_ty(abi, &param_tys, fn_ty.is_variadic, return_ty)
        }
        ast::Ty::Item(item_ty) => {
            match item_ty.kind {
                ast::ItemTyKind::Alias => tys::alias,
                ast::ItemTyKind::Module => tys::module,
            }
        }
        ast::Ty::ManyPtr(many_ptr_ty) => {
            let pointee_ty = resolve_ty(context, module_id, ast_file, many_ptr_ty.pointee_ty)?;
            context.add_many_ptr_ty(pointee_ty, many_ptr_ty.is_mutable)
        }
        ast::Ty::Paren(paren_ty) => resolve_ty(context, module_id, ast_file, paren_ty.inner_ty)?,
        ast::Ty::Path(path_ty) => {
            let path = path_ty
                .idents
                .iter()
                .map(|ident| ident.symbol)
                .collect::<SmallVec<_, 8>>();

            let item_id = context.resolve_path(module_id, &path)?;

            let Item::Ty(ty_id) = context[item_id] else {
                return Err(ResolveError::ItemNotTy { item_id });
            };

            ty_id
        }
        ast::Ty::Ptr(ptr_ty) => {
            let pointee_ty = resolve_ty(context, module_id, ast_file, ptr_ty.pointee_ty)?;
            context.add_ptr_ty(pointee_ty, ptr_ty.is_mutable)
        }
        ast::Ty::Slice(slice_ty) => {
            let elem_ty = resolve_ty(context, module_id, ast_file, slice_ty.elem_ty)?;
            context.add_slice_ty(elem_ty, slice_ty.is_mutable)
        }
        ast::Ty::Tuple(tuple_ty) => {
            let elem_tys = tuple_ty
                .elem_tys
                .iter()
                .map(|&ty_id| resolve_ty(context, module_id, ast_file, ty_id))
                .collect::<Result<SmallVec<_, 8>, _>>()?;

            context.add_tuple_ty(&elem_tys)
        }
        ast::Ty::Variant(variant_ty) => {
            let variant_tys = variant_ty
                .variant_tys
                .iter()
                .map(|&ty_id| resolve_ty(context, module_id, ast_file, ty_id))
                .collect::<Result<SmallVec<_, 8>, _>>()?;

            context.add_variant_ty(&variant_tys)
        }
    };

    Ok(ty_id)
}
