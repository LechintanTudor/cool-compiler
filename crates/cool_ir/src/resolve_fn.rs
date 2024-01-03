use crate::resolve_ty;
use cool_ast as ast;
use cool_collections::SmallVec;
use cool_lexer::sym;
use cool_resolve::{
    tys, FnAbi, FnTy, ModuleId, ResolveContext, ResolveError, ResolveResult, TyId, TyKind,
};

pub fn resolve_fn(
    context: &mut ResolveContext,
    module_id: ModuleId,
    ty_id: TyId,
    ast_file: &ast::File,
    ast_fn_proto_id: ast::FnProtoId,
) -> ResolveResult<TyId> {
    if ty_id == tys::infer {
        resolve_inferred_fn(context, module_id, ast_file, ast_fn_proto_id)
    } else {
        let fn_ty = match &context[ty_id] {
            TyKind::Fn(fn_ty) => fn_ty.clone(),
            _ => return Err(ResolveError::TyNotFn { ty_id }),
        };

        resolve_hinted_fn(context, module_id, &fn_ty, ast_file, ast_fn_proto_id)?;
        Ok(ty_id)
    }
}

fn resolve_inferred_fn(
    context: &mut ResolveContext,
    module_id: ModuleId,
    ast_file: &ast::File,
    ast_fn_proto_id: ast::FnProtoId,
) -> ResolveResult<TyId> {
    let fn_proto = &ast_file[ast_fn_proto_id];

    let abi = match fn_proto.abi {
        ast::FnAbi::Implicit => FnAbi::Cool,
        ast::FnAbi::Explicit(None) => FnAbi::C,
        ast::FnAbi::Explicit(Some(sym::C)) => FnAbi::C,
        ast::FnAbi::Explicit(Some(sym::Cool)) => FnAbi::Cool,
        ast::FnAbi::Explicit(Some(abi)) => {
            return Err(ResolveError::FnAbiIsUnknown { abi });
        }
    };

    let mut param_tys = SmallVec::<TyId, 8>::new();

    for param in &fn_proto.params {
        let Some(param_ast_ty_id) = param.ty else {
            return Err(ResolveError::FnParamTyMissing);
        };

        let param_ty = resolve_ty(context, module_id, ast_file, param_ast_ty_id)?;
        param_tys.push(param_ty);
    }

    let return_ty = fn_proto
        .return_ty
        .map(|ty| resolve_ty(context, module_id, ast_file, ty))
        .transpose()?
        .unwrap_or(tys::unit);

    Ok(context.add_fn_ty(abi, &param_tys, fn_proto.is_variadic, return_ty))
}

fn resolve_hinted_fn(
    context: &mut ResolveContext,
    module_id: ModuleId,
    fn_ty: &FnTy,
    ast_file: &ast::File,
    ast_fn_proto_id: ast::FnProtoId,
) -> ResolveResult {
    let fn_proto = &ast_file[ast_fn_proto_id];

    match fn_ty.abi {
        FnAbi::C => {
            if !matches!(
                fn_proto.abi,
                ast::FnAbi::Implicit | ast::FnAbi::Explicit(None | Some(sym::C)),
            ) {
                return Err(ResolveError::FnAbiMismatch);
            }
        }
        FnAbi::Cool => {
            if !matches!(
                fn_proto.abi,
                ast::FnAbi::Implicit | ast::FnAbi::Explicit(Some(sym::Cool)),
            ) {
                return Err(ResolveError::FnAbiMismatch);
            }
        }
    }

    if fn_ty.param_tys.len() != fn_proto.params.len() {
        return Err(ResolveError::FnParamCountMismatch {
            found: fn_ty.param_tys.len() as u32,
            expected: fn_proto.params.len() as u32,
        });
    }

    for (&param_ty, param) in fn_ty.param_tys.iter().zip(&fn_proto.params) {
        if let Some(param_ast_ty) = param.ty {
            let expr_param_ty = resolve_ty(context, module_id, ast_file, param_ast_ty)?;

            if param_ty != expr_param_ty {
                return Err(ResolveError::FnParamTyMimatch {
                    found: expr_param_ty,
                    expected: param_ty,
                });
            }
        }
    }

    if fn_ty.is_variadic != fn_proto.is_variadic {
        return Err(ResolveError::FnVariadicMismatch {
            found: fn_ty.is_variadic,
        });
    }

    if let Some(return_ast_ty) = fn_proto.return_ty {
        let return_ty = resolve_ty(context, module_id, ast_file, return_ast_ty)?;

        if fn_ty.return_ty != return_ty {
            return Err(ResolveError::FnReturnTyMismatch {
                found: return_ty,
                expected: fn_ty.return_ty,
            });
        }
    }

    Ok(())
}
