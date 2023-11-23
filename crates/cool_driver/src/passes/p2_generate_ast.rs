use crate::{DefinedCrate, SpannedCompileError, WithLocation};
use cool_ast::AstGenerator;
use cool_resolve::{ResolveContext, ResolveError};

pub fn p2_generate_ast(
    defined_crate: &DefinedCrate,
    context: &mut ResolveContext<'static>,
    errors: &mut Vec<SpannedCompileError>,
) {
    let mut ast = AstGenerator::new(context);

    for fn_item in defined_crate.fns.iter() {
        let context = ast.context();

        let ty_id = match context[fn_item.item_id]
            .try_as_const()
            .map(|const_id| context[const_id].ty_id)
            .ok_or(ResolveError::ItemNotConst {
                item_id: fn_item.item_id,
            })
            .with_location((fn_item.source_id, fn_item.span))
        {
            Ok(ty_id) => ty_id,
            Err(error) => {
                errors.push(error);
                continue;
            }
        };

        if let Err(error) = ast
            .gen_fn_expr(&fn_item.item, fn_item.module_id, ty_id)
            .map_err(|spanned_error| {
                SpannedCompileError {
                    location: (fn_item.source_id, spanned_error.span).into(),
                    error: spanned_error.error.into(),
                }
            })
        {
            errors.push(error);
        }
    }
}
