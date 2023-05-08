use crate::{
    AstResult, FnAbiMismatch, FnParamCountMismatch, FnVariadicMismatch, TyHintMissing, TyMismatch,
    TyNotFn,
};
use cool_parser::{FnExternDecl, FnPrototype, Ty};
use cool_resolve::{
    tys, FnAbi, ItemPathBuf, ModuleId, ResolveContext, ResolveError, ResolveResult, ScopeId, TyId,
};
use smallvec::SmallVec;

pub fn resolve_fn_abi(extern_decl: &Option<FnExternDecl>) -> ResolveResult<FnAbi> {
    let abi = match extern_decl.as_ref() {
        Some(decl) => {
            match decl.abi {
                Some(abi) => {
                    match FnAbi::try_from(abi) {
                        Ok(abi) => abi,
                        Err(_) => return Err(ResolveError::not_abi(abi)),
                    }
                }
                None => FnAbi::C,
            }
        }
        None => FnAbi::Cool,
    };

    Ok(abi)
}

pub fn resolve_explicit_fn_abi(extern_decl: &Option<FnExternDecl>) -> ResolveResult<Option<FnAbi>> {
    let abi = match extern_decl.as_ref() {
        Some(decl) => {
            match decl.abi {
                Some(abi) => {
                    match FnAbi::try_from(abi) {
                        Ok(abi) => Some(abi),
                        Err(_) => return Err(ResolveError::not_abi(abi)),
                    }
                }
                None => Some(FnAbi::C),
            }
        }
        None => None,
    };

    Ok(abi)
}

pub fn resolve_fn_prototype(
    resolve: &mut ResolveContext,
    module_id: ModuleId,
    explicit_ty: &Option<Ty>,
    prototype: &FnPrototype,
) -> AstResult<TyId> {
    let scope_id = ScopeId::Module(module_id);

    let explicit_ty_id = explicit_ty
        .as_ref()
        .map(|ty| resolve_ty(resolve, scope_id, ty))
        .transpose()?;

    match explicit_ty_id {
        Some(ty_id) => {
            let fn_ty = resolve[ty_id]
                .kind
                .as_fn_ty()
                .ok_or(TyNotFn { found: ty_id })?
                .clone();

            if let Some(abi) = resolve_explicit_fn_abi(&prototype.extern_decl)? {
                if abi != fn_ty.abi {
                    Err(FnAbiMismatch {
                        found: abi,
                        expected: fn_ty.abi,
                    })?;
                }
            }

            if prototype.param_list.params.len() != fn_ty.params.len() {
                Err(FnParamCountMismatch {
                    found: prototype.param_list.params.len() as _,
                    expected: fn_ty.params.len() as _,
                })?;
            }

            for (i, param) in prototype.param_list.params.iter().enumerate() {
                let param_ty = match param.ty.as_ref() {
                    Some(param_ty) => param_ty,
                    None => continue,
                };

                let param_ty_id = resolve_ty(resolve, scope_id, param_ty)?;

                if param_ty_id != fn_ty.params[i] {
                    Err(TyMismatch {
                        found: param_ty_id,
                        expected: fn_ty.params[i],
                    })?;
                }
            }

            if prototype.param_list.is_variadic != fn_ty.is_variadic {
                Err(FnVariadicMismatch {
                    found: prototype.param_list.is_variadic,
                    expected: fn_ty.is_variadic,
                })?;
            }

            if let Some(ret_ty) = prototype.ret_ty.as_ref() {
                let ret_ty_id = resolve_ty(resolve, scope_id, ret_ty)?;

                if ret_ty_id != fn_ty.ret {
                    Err(TyMismatch {
                        found: ret_ty_id,
                        expected: fn_ty.ret,
                    })?;
                }
            }

            Ok(ty_id)
        }
        None => {
            let abi = resolve_fn_abi(&prototype.extern_decl)?;
            let mut param_ty_ids = SmallVec::<[TyId; 4]>::new();

            for param in prototype.param_list.params.iter() {
                let param_ty = param.ty.as_ref().ok_or(TyHintMissing)?;
                let param_ty_id = resolve_ty(resolve, scope_id, param_ty)?;
                param_ty_ids.push(param_ty_id);
            }

            let ret_ty_id = match prototype.ret_ty.as_ref() {
                Some(ret_ty) => resolve_ty(resolve, scope_id, ret_ty)?,
                None => tys::UNIT,
            };

            Ok(resolve.mk_fn(
                abi,
                param_ty_ids,
                prototype.param_list.is_variadic,
                ret_ty_id,
            ))
        }
    }
}

pub fn resolve_ty(resolve: &mut ResolveContext, scope_id: ScopeId, ty: &Ty) -> ResolveResult<TyId> {
    match ty {
        Ty::Fn(fn_ty) => {
            let abi = resolve_fn_abi(&fn_ty.extern_decl)?;
            let mut param_ty_ids = SmallVec::<[TyId; 6]>::new();

            for param in fn_ty.param_list.params.iter() {
                param_ty_ids.push(resolve_ty(resolve, scope_id, param)?);
            }

            let ret_ty_id = match &fn_ty.ret_ty {
                Some(ret_ty) => resolve_ty(resolve, scope_id, ret_ty)?,
                None => tys::UNIT,
            };

            Ok(resolve.mk_fn(abi, param_ty_ids, fn_ty.param_list.is_variadic, ret_ty_id))
        }
        Ty::Path(path) => {
            let path = path
                .idents
                .iter()
                .map(|ident| ident.symbol)
                .collect::<ItemPathBuf>();

            let item_id = resolve.resolve_global(scope_id, &path)?;

            resolve[item_id]
                .as_ty_id()
                .ok_or(ResolveError::not_ty(path.last()))
        }
        Ty::Pointer(pointer_ty) => {
            let pointee = resolve_ty(resolve, scope_id, &pointer_ty.pointee)?;
            Ok(resolve.mk_pointer(pointer_ty.is_mutable, pointee))
        }
        Ty::Tuple(tuple_ty) => {
            let mut elem_tys = SmallVec::<[TyId; 6]>::new();

            for ty in tuple_ty.elems.iter() {
                elem_tys.push(resolve_ty(resolve, scope_id, ty)?);
            }

            Ok(resolve.mk_tuple(elem_tys))
        }
        _ => todo!(),
    }
}
