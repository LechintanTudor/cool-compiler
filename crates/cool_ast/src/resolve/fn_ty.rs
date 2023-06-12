use crate::{
    AstError, AstGenerator, AstResult, FnAbiMismatch, FnParamCountMismatch, FnVariadicMismatch,
    TyNotFn,
};
use cool_parser::{FnExternDecl, FnPrototype, Ty};
use cool_resolve::{
    FnAbi, ModuleId, ResolveError, ResolveErrorKind, ResolveResult, Scope, TyId, TyMismatch,
};
use smallvec::SmallVec;

impl AstGenerator<'_> {
    pub fn resolve_fn_prototype(
        &mut self,
        module_id: ModuleId,
        explicit_ty: &Option<Ty>,
        prototype: &FnPrototype,
    ) -> AstResult<TyId> {
        let scope = Scope::Module(module_id);

        let explicit_ty_id = explicit_ty
            .as_ref()
            .map(|ty| self.resolve_ty(scope, ty))
            .transpose()?;

        match explicit_ty_id {
            Some(ty_id) => {
                let fn_ty = ty_id.as_fn().ok_or(TyNotFn { found: ty_id })?.clone();

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

                    let param_ty_id = self.resolve_ty(scope, param_ty)?;

                    if param_ty_id != fn_ty.params[i] {
                        Err(TyMismatch {
                            found_ty_id: param_ty_id,
                            expected_ty_id: fn_ty.params[i],
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
                    let ret_ty_id = self.resolve_ty(scope, ret_ty)?;

                    if ret_ty_id != fn_ty.ret {
                        Err(TyMismatch {
                            found_ty_id: ret_ty_id,
                            expected_ty_id: fn_ty.ret,
                        })?;
                    }
                }

                Ok(ty_id)
            }
            None => {
                let abi = resolve_fn_abi(&prototype.extern_decl)?;
                let mut param_ty_ids = SmallVec::<[TyId; 4]>::new();

                for param in prototype.param_list.params.iter() {
                    let param_ty = param.ty.as_ref().ok_or(AstError::TyHintMissing)?;
                    let param_ty_id = self.resolve_ty(scope, param_ty)?;
                    param_ty_ids.push(param_ty_id);
                }

                let ret_ty_id = match prototype.ret_ty.as_ref() {
                    Some(ret_ty) => self.resolve_ty(scope, ret_ty)?,
                    None => self.tys().unit,
                };

                Ok(self.resolve.mk_fn(
                    abi,
                    param_ty_ids,
                    prototype.param_list.is_variadic,
                    ret_ty_id,
                ))
            }
        }
    }
}

pub fn resolve_fn_abi(extern_decl: &Option<FnExternDecl>) -> ResolveResult<FnAbi> {
    let abi = match extern_decl.as_ref() {
        Some(decl) => {
            match decl.abi {
                Some(abi) => {
                    match FnAbi::try_from(abi) {
                        Ok(abi) => abi,
                        Err(_) => {
                            return Err(ResolveError {
                                symbol: abi,
                                kind: ResolveErrorKind::SymbolNotAbi,
                            });
                        }
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
                        Err(_) => {
                            return Err(ResolveError {
                                symbol: abi,
                                kind: ResolveErrorKind::SymbolNotAbi,
                            });
                        }
                    }
                }
                None => Some(FnAbi::C),
            }
        }
        None => None,
    };

    Ok(abi)
}
