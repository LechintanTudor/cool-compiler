use crate::Package;
use cool_ast::{resolve_fn_prototype, resolve_ty};
use cool_parser::{Expr};
use cool_resolve::{ResolveContext, ScopeId};


pub fn p2_gen_ast(package: &Package, resolve: &mut ResolveContext) {
    for const_item in package.consts.iter() {
        match &const_item.item.expr {
            Expr::Fn(fn_expr) => {
                let scope_id = ScopeId::Module(const_item.module_id);
                let explicit_ty_id = match const_item.ty.as_ref() {
                    Some(ty) => Some(resolve_ty(resolve, scope_id, ty).unwrap()),
                    None => None,
                };

                let fn_expr_ty_id = resolve_fn_prototype(
                    resolve,
                    const_item.module_id,
                    explicit_ty_id,
                    &fn_expr.prototype,
                )
                .unwrap();

                let binding_id = resolve[const_item.item_id].as_binding_id().unwrap();
                resolve.set_binding_ty(binding_id, fn_expr_ty_id);
            }
            _ => (),
        }
    }
}
