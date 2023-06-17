use crate::{CompileError, CompileErrorBundle, CompileResult, Package};
use cool_ast::{AstGenerator, ExternFnAst, FnAst, PackageAst};
use cool_parser::Expr;
use cool_resolve::ResolveContext;

pub fn p4_gen_ast(package: &Package, resolve: &mut ResolveContext) -> CompileResult<PackageAst> {
    let mut ast = AstGenerator::new(resolve);
    let mut errors = Vec::<CompileError>::new();
    let mut extern_fns = Vec::<ExternFnAst>::new();
    let mut fns = Vec::<FnAst>::new();

    for extern_fn in package.extern_fns.iter() {
        let extern_fn_binding_id = ast.resolve[extern_fn.item_id].as_binding_id().unwrap();
        let extern_fn_ty_id = ast.resolve[extern_fn_binding_id].ty_id;
        let extern_fn_ast = match ast.gen_extern_fn(extern_fn.item_id, extern_fn_ty_id) {
            Ok(extern_fn_ast) => extern_fn_ast,
            Err(error) => {
                errors.push(error.into());
                continue;
            }
        };

        extern_fns.push(extern_fn_ast);
    }

    for const_item in package.consts.iter() {
        if let Expr::Fn(fn_expr) = &const_item.item.expr {
            let fn_binding_id = ast.resolve[const_item.item_id].as_binding_id().unwrap();
            let fn_ty_id = ast.resolve[fn_binding_id].ty_id;
            let fn_ast =
                match ast.gen_fn(const_item.item_id, const_item.module_id, fn_ty_id, fn_expr) {
                    Ok(fn_ast) => fn_ast,
                    Err(error) => {
                        errors.push(error.into());
                        continue;
                    }
                };

            fns.push(fn_ast);
        }
    }

    if errors.is_empty() {
        Ok(PackageAst {
            fns,
            extern_fns,
            defer_stmts: ast.defer_codes,
        })
    } else {
        Err(CompileErrorBundle { errors })
    }
}
