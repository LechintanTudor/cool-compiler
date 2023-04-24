use crate::{CompileError, CompileErrorBundle, CompileResult, Package};
use cool_ast::resolve_fn_prototype;
use cool_parser::Expr;
use cool_resolve::ResolveContext;

pub fn p2_define_fn_tys(package: &Package, resolve: &mut ResolveContext) -> CompileResult<()> {
    let mut errors = Vec::<CompileError>::new();

    for extern_fn in package.extern_fns.iter() {
        let fn_expr_ty_id = match resolve_fn_prototype(
            resolve,
            extern_fn.module_id,
            &extern_fn.ty,
            &extern_fn.item.prototype,
        ) {
            Ok(fn_expr_ty_id) => fn_expr_ty_id,
            Err(error) => {
                errors.push(CompileError {
                    path: Default::default(),
                    kind: error.into(),
                });
                continue;
            }
        };
        let binding_id = resolve[extern_fn.item_id].as_binding_id().unwrap();
        resolve.set_binding_ty(binding_id, fn_expr_ty_id);
    }

    for const_item in package.consts.iter() {
        match &const_item.item.expr {
            Expr::Fn(fn_expr) => {
                let fn_expr_ty_id = match resolve_fn_prototype(
                    resolve,
                    const_item.module_id,
                    &const_item.ty,
                    &fn_expr.prototype,
                ) {
                    Ok(fn_expr_ty_id) => fn_expr_ty_id,
                    Err(error) => {
                        errors.push(CompileError {
                            path: Default::default(),
                            kind: error.into(),
                        });
                        continue;
                    }
                };

                let binding_id = resolve[const_item.item_id].as_binding_id().unwrap();
                resolve.set_binding_ty(binding_id, fn_expr_ty_id);
            }
            _ => (),
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(CompileErrorBundle { errors })
    }
}
