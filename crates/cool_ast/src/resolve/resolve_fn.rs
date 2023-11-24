use crate::{resolve_ty, SpannedAstError, SpannedAstResult, WithSpan};
use cool_collections::SmallVec;
use cool_parser::{FnAbiDecl, FnExprPrototype};
use cool_resolve::{tys, FnAbi, ModuleId, ResolveContext, ResolveError, Scope, TyId};
use cool_span::Section;

pub fn resolve_fn(
    context: &mut ResolveContext,
    module_id: ModuleId,
    ty_id: Option<TyId>,
    fn_prototype: &FnExprPrototype,
) -> SpannedAstResult<TyId> {
    let scope = Scope::Module(module_id);

    let fn_ty = ty_id
        .map(|ty_id| {
            context[ty_id]
                .try_as_fn()
                .ok_or(ResolveError::TyNotFn { ty_id })
        })
        .transpose()
        .with_span(fn_prototype.span)?
        .cloned();

    let ty_id = match fn_ty {
        Some(fn_ty) => {
            if let Some(expr_abi) = resolve_fn_expr_abi(fn_prototype.abi_decl.as_ref())? {
                if fn_ty.abi != expr_abi {
                    return Err(SpannedAstError::new(
                        fn_prototype.span,
                        ResolveError::FnAbiMismatch {
                            found: expr_abi,
                            expected: fn_ty.abi,
                        },
                    ));
                }
            }

            if fn_ty.param_tys.len() != fn_prototype.params.len() {
                return Err(SpannedAstError::new(
                    fn_prototype.span,
                    ResolveError::FnParamCountMismatch {
                        found: fn_ty.param_tys.len() as _,
                        expected: fn_prototype.params.len() as _,
                    },
                ));
            }

            let params_to_check = fn_ty
                .param_tys
                .iter()
                .zip(fn_prototype.params.iter())
                .flat_map(|(param_ty, param)| param.ty.as_ref().map(|ty| (*param_ty, ty)));

            for (param_ty, expr_ty) in params_to_check {
                let expr_ty = resolve_ty(context, scope, expr_ty)?;

                if param_ty != expr_ty {
                    return Err(SpannedAstError::new(
                        fn_prototype.span,
                        ResolveError::FnParamTyMimatch {
                            found: expr_ty,
                            expected: param_ty,
                        },
                    ));
                }
            }

            if fn_ty.is_variadic != fn_prototype.is_variadic {
                return Err(SpannedAstError::new(
                    fn_prototype.span,
                    ResolveError::FnVariadicMismatch {
                        found: fn_prototype.is_variadic,
                    },
                ));
            }

            if let Some(expr_return_ty) = fn_prototype.return_ty.as_ref() {
                let expr_return_ty = resolve_ty(context, scope, expr_return_ty)?;

                if fn_ty.return_ty != expr_return_ty {
                    return Err(SpannedAstError::new(
                        fn_prototype.span,
                        ResolveError::FnReturnTyMismatch {
                            found: expr_return_ty,
                            expected: fn_ty.return_ty,
                        },
                    ));
                }
            }

            ty_id.unwrap()
        }
        None => {
            let abi = resolve_fn_ty_abi(fn_prototype.abi_decl.as_ref())?;

            let param_tys = {
                let mut param_tys = SmallVec::<TyId, 12>::new();

                for param in fn_prototype.params.iter() {
                    let Some(param_ty) = param.ty.as_ref() else {
                        return Err(SpannedAstError::new(
                            param.span(),
                            ResolveError::FnParamTyMissing,
                        ));
                    };

                    param_tys.push(resolve_ty(context, scope, param_ty)?);
                }

                param_tys
            };

            let return_ty = fn_prototype
                .return_ty
                .as_ref()
                .map(|ty| resolve_ty(context, scope, ty))
                .transpose()?
                .unwrap_or(tys::unit);

            context.add_fn_ty(abi, param_tys, fn_prototype.is_variadic, return_ty)
        }
    };

    Ok(ty_id)
}

fn resolve_fn_expr_abi(abi_decl: Option<&FnAbiDecl>) -> SpannedAstResult<Option<FnAbi>> {
    let Some(abi_decl) = abi_decl else {
        return Ok(None);
    };

    let abi = abi_decl
        .abi
        .map(FnAbi::try_from)
        .transpose()
        .with_span(abi_decl.span)?
        .unwrap_or(FnAbi::C);

    Ok(Some(abi))
}

fn resolve_fn_ty_abi(abi_decl: Option<&FnAbiDecl>) -> SpannedAstResult<FnAbi> {
    let Some(abi_decl) = abi_decl else {
        return Ok(FnAbi::Cool);
    };

    let abi = abi_decl
        .abi
        .map(FnAbi::try_from)
        .transpose()
        .with_span(abi_decl.span)?
        .unwrap_or(FnAbi::C);

    Ok(abi)
}
