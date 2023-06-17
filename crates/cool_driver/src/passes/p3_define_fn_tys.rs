use crate::{CompileError, CompileErrorBundle, CompileResult, Package};
use cool_ast::AstGenerator;
use cool_parser::Expr;
use cool_resolve::ResolveContext;

pub fn p3_define_fn_tys(package: &Package, resolve: &mut ResolveContext) -> CompileResult<()> {
    let mut ast = AstGenerator::new(resolve);
    let mut errors = Vec::<CompileError>::new();

    for extern_fn in package.extern_fns.iter() {
        let fn_expr_ty_id = match ast.resolve_fn_prototype(
            extern_fn.module_id,
            &extern_fn.ty,
            &extern_fn.item.prototype,
        ) {
            Ok(fn_expr_ty_id) => fn_expr_ty_id,
            Err(error) => {
                errors.push(error.into());
                continue;
            }
        };

        let binding_id = ast.resolve[extern_fn.item_id].as_binding_id().unwrap();
        ast.resolve.set_binding_ty(binding_id, fn_expr_ty_id);
    }

    for const_item in package.consts.iter() {
        if let Expr::Fn(fn_expr) = &const_item.item.expr {
            let fn_expr_ty_id = match ast.resolve_fn_prototype(
                const_item.module_id,
                &const_item.ty,
                &fn_expr.prototype,
            ) {
                Ok(fn_expr_ty_id) => fn_expr_ty_id,
                Err(error) => {
                    errors.push(error.into());
                    continue;
                }
            };

            let binding_id = ast.resolve[const_item.item_id].as_binding_id().unwrap();
            ast.resolve.set_binding_ty(binding_id, fn_expr_ty_id);
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(CompileErrorBundle { errors })
    }
}
