use crate::{AstError, AstGenerator, AstResult, AstResultExt, TyDefError, TyError, TyErrorKind};
use cool_parser::{FnExternDecl, FnPrototype, Ty};
use cool_resolve::{FnAbi, ModuleId, Scope, TyId};
use cool_span::Section;
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
                let Some(fn_ty) = ty_id.as_fn() else {
                    return AstResult::error(prototype.span(), TyError {
                        ty_id,
                        kind: TyErrorKind::TyNotCallable,
                    });
                };

                if let Some(abi) = resolve_explicit_fn_abi(&prototype.extern_decl)? {
                    if abi != fn_ty.abi {
                        return AstResult::error(
                            prototype.span(),
                            TyDefError::AbiMismatch {
                                found: abi,
                                expected: fn_ty.abi,
                            },
                        );
                    }
                }

                if prototype.param_list.params.len() != fn_ty.params.len() {
                    return AstResult::error(
                        prototype.span(),
                        TyDefError::ParamCountMismatch {
                            found: prototype.param_list.params.len() as _,
                            expected: fn_ty.params.len() as _,
                        },
                    );
                }

                for (i, param) in prototype.param_list.params.iter().enumerate() {
                    let param_ty = match param.ty.as_ref() {
                        Some(param_ty) => param_ty,
                        None => continue,
                    };

                    let param_ty_id = self.resolve_ty(scope, param_ty)?;

                    if param_ty_id != fn_ty.params[i] {
                        return AstResult::ty_mismatch(param.span(), param_ty_id, fn_ty.params[i]);
                    }
                }

                if prototype.param_list.is_variadic != fn_ty.is_variadic {
                    return AstResult::error(
                        prototype.span(),
                        TyDefError::VariadicMismatch {
                            found: prototype.param_list.is_variadic,
                            expected: fn_ty.is_variadic,
                        },
                    );
                }

                if let Some(ret_ty) = prototype.ret_ty.as_ref() {
                    let ret_ty_id = self.resolve_ty(scope, ret_ty)?;

                    if ret_ty_id != fn_ty.ret {
                        return AstResult::ty_mismatch(ret_ty.span(), ret_ty_id, fn_ty.ret);
                    }
                }

                Ok(ty_id)
            }
            None => {
                let abi = resolve_fn_abi(&prototype.extern_decl)?;
                let mut param_ty_ids = SmallVec::<[TyId; 4]>::new();

                for param in prototype.param_list.params.iter() {
                    let param_ty = param.ty.as_ref().ok_or_else(|| {
                        AstError::new(
                            param.span(),
                            TyDefError::TyHintMissing {
                                param: param.ident.symbol,
                            },
                        )
                    })?;

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

pub fn resolve_fn_abi(extern_decl: &Option<FnExternDecl>) -> AstResult<FnAbi> {
    let Some(decl) = extern_decl.as_ref() else {
        return Ok(FnAbi::Cool);
    };

    let Some(abi_symbol) = decl.abi else {
        return Ok(FnAbi::C);
    };

    FnAbi::try_from(abi_symbol)
        .map_err(|_| AstError::new(decl.span(), TyDefError::UnknownAbi { abi: abi_symbol }))
}

pub fn resolve_explicit_fn_abi(extern_decl: &Option<FnExternDecl>) -> AstResult<Option<FnAbi>> {
    let Some(decl) = extern_decl.as_ref() else {
        return Ok(None);
    };

    let Some(abi_symbol) = decl.abi else {
        return Ok(Some(FnAbi::C));
    };

    FnAbi::try_from(abi_symbol)
        .map(Some)
        .map_err(|_| AstError::new(decl.span(), TyDefError::UnknownAbi { abi: abi_symbol }))
}
