use crate::expr::{BlockExprAst, GenericExprAst};
use crate::{
    AstGenerator, AstResult, FnAbiMismatch, FnParamAst, FnPrototypeAst, InvalidArgCount,
    ResolveAst, TyHintMissing, TyMismatch,
};
use cool_lexer::symbols::sym;
use cool_parser::FnExpr;
use cool_resolve::{tys, ExprId, FrameId, ModuleId, TyId, TyKind};
use smallvec::SmallVec;

#[derive(Clone, Debug)]
pub struct FnExprAst {
    pub id: ExprId,
    pub frame_id: FrameId,
    pub prototype: FnPrototypeAst,
    pub body: BlockExprAst,
}

impl GenericExprAst for FnExprAst {
    #[inline]
    fn id(&self) -> ExprId {
        self.id
    }
}

impl ResolveAst for FnExprAst {
    fn resolve_consts(&self, ast: &mut AstGenerator, expected_ty: TyId) -> AstResult<TyId> {
        let (abi, param_ty_ids, ret_ty_id) = match ast.resolve[expected_ty].clone() {
            TyKind::Inferred(_) => {
                let abi = self.prototype.abi.unwrap_or(sym::ABI_COOL);

                let mut param_ty_ids = SmallVec::<[TyId; 6]>::new();

                for param in self.prototype.params.iter() {
                    let param_ty_id = param
                        .ty_id
                        .resolve_non_inferred(tys::INFERRED)
                        .ok_or(TyHintMissing)?;

                    param_ty_ids.push(param_ty_id);
                }

                (
                    abi,
                    param_ty_ids,
                    self.prototype.ret_ty_id.unwrap_or(tys::UNIT),
                )
            }
            TyKind::Fn(fn_ty) => {
                let abi = match self.prototype.abi {
                    Some(abi) => {
                        if fn_ty.abi == abi {
                            abi
                        } else {
                            Err(FnAbiMismatch {
                                found: abi,
                                expected: fn_ty.abi,
                            })?
                        }
                    }
                    None => fn_ty.abi,
                };

                if self.prototype.params.len() != fn_ty.params.len() {
                    Err(InvalidArgCount {
                        found: self.prototype.params.len() as _,
                        expected: fn_ty.params.len() as _,
                    })?
                }

                let mut param_ty_ids = SmallVec::<[TyId; 6]>::new();

                let param_iter = {
                    let params = self.prototype.params.iter();
                    let hint_ty_ids = fn_ty.params.iter();
                    params.zip(hint_ty_ids)
                };

                for (param, &hint_ty_id) in param_iter {
                    let param_ty_id =
                        param
                            .ty_id
                            .resolve_non_inferred(hint_ty_id)
                            .ok_or(TyMismatch {
                                found_ty: param.ty_id,
                                expected_ty: hint_ty_id,
                            })?;

                    param_ty_ids.push(param_ty_id);
                }

                let ret_ty_id = {
                    let ret_ty_id = self.prototype.ret_ty_id.unwrap_or(tys::INFERRED);

                    ret_ty_id
                        .resolve_non_inferred(fn_ty.ret)
                        .ok_or(TyMismatch {
                            found_ty: ret_ty_id,
                            expected_ty: fn_ty.ret,
                        })?
                };

                (abi, param_ty_ids, ret_ty_id)
            }
            _ => panic!("hint type is not a function"),
        };

        let fn_ty_id = ast.resolve.mk_fn(abi, param_ty_ids, ret_ty_id);
        ast.resolve.set_expr_ty(self.id, fn_ty_id);
        Ok(fn_ty_id)
    }

    fn resolve_exprs(&self, ast: &mut AstGenerator, expected_ty: TyId) -> AstResult<TyId> {
        let (abi, param_ty_ids, ret_ty_id) = match ast.resolve[expected_ty].clone() {
            TyKind::Inferred(_) => {
                let abi = self.prototype.abi.unwrap_or(sym::ABI_COOL);

                let mut param_ty_ids = SmallVec::<[TyId; 6]>::new();

                for param in self.prototype.params.iter() {
                    let param_ty_id = param
                        .ty_id
                        .resolve_non_inferred(tys::INFERRED)
                        .ok_or(TyHintMissing)?;

                    param_ty_ids.push(param_ty_id);
                    ast.resolve.set_binding_ty(param.binding_id, param_ty_id);
                }

                (
                    abi,
                    param_ty_ids,
                    self.prototype.ret_ty_id.unwrap_or(tys::UNIT),
                )
            }
            TyKind::Fn(fn_ty) => {
                let abi = match self.prototype.abi {
                    Some(abi) => {
                        if fn_ty.abi == abi {
                            abi
                        } else {
                            Err(FnAbiMismatch {
                                found: abi,
                                expected: fn_ty.abi,
                            })?
                        }
                    }
                    None => fn_ty.abi,
                };

                if self.prototype.params.len() != fn_ty.params.len() {
                    Err(InvalidArgCount {
                        found: self.prototype.params.len() as _,
                        expected: fn_ty.params.len() as _,
                    })?
                }

                let mut param_ty_ids = SmallVec::<[TyId; 6]>::new();

                let param_iter = {
                    let params = self.prototype.params.iter();
                    let hint_ty_ids = fn_ty.params.iter();
                    params.zip(hint_ty_ids)
                };

                for (param, &hint_ty_id) in param_iter {
                    let param_ty_id =
                        param
                            .ty_id
                            .resolve_non_inferred(hint_ty_id)
                            .ok_or(TyMismatch {
                                found_ty: param.ty_id,
                                expected_ty: hint_ty_id,
                            })?;

                    param_ty_ids.push(param_ty_id);
                    ast.resolve.set_binding_ty(param.binding_id, param_ty_id);
                }

                let ret_ty_id = {
                    let ret_ty_id = self.prototype.ret_ty_id.unwrap_or(tys::INFERRED);

                    ret_ty_id
                        .resolve_non_inferred(fn_ty.ret)
                        .ok_or(TyMismatch {
                            found_ty: ret_ty_id,
                            expected_ty: fn_ty.ret,
                        })?
                };

                (abi, param_ty_ids, ret_ty_id)
            }
            _ => panic!("hint type is not a function"),
        };

        let fn_ty_id = ast.resolve.mk_fn(abi, param_ty_ids, ret_ty_id);
        ast.resolve.set_expr_ty(self.id, fn_ty_id);
        self.body.resolve_exprs(ast, ret_ty_id)?;
        Ok(fn_ty_id)
    }
}

impl AstGenerator<'_> {
    pub fn gen_fn(&mut self, module_id: ModuleId, fn_expr: &FnExpr) -> FnExprAst {
        let abi = fn_expr
            .prototype
            .extern_decl
            .as_ref()
            .map(|decl| decl.abi.unwrap_or(sym::ABI_C));

        let frame_id = self.resolve.add_frame(module_id.into());
        let mut params = SmallVec::new();

        for param in fn_expr.prototype.param_list.params.iter() {
            let ty_id = match param.ty.as_ref() {
                Some(ty) => self.resolve_parsed_ty(module_id.into(), ty).unwrap(),
                None => tys::INFERRED,
            };

            let binding_id = self
                .resolve
                .insert_local_binding(frame_id, param.is_mutable, param.ident.symbol)
                .unwrap();

            params.push(FnParamAst { binding_id, ty_id });
        }

        let ret_ty_id = match fn_expr.prototype.ret_ty.as_ref() {
            Some(ty) => Some(self.resolve_parsed_ty(module_id.into(), ty).unwrap()),
            None => None,
        };

        let body = self.gen_block_expr(frame_id.into(), &fn_expr.body);

        FnExprAst {
            id: self.resolve.add_expr(),
            frame_id,
            prototype: FnPrototypeAst {
                abi,
                params,
                ret_ty_id,
            },
            body,
        }
    }
}
