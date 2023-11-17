use crate::DefinedCrate;
use cool_ast::{AstGenerator, AstResult};
use cool_resolve::{ResolveContext, ResolveError};

pub fn p2_generate_ast(
    defined_crate: &DefinedCrate,
    context: &mut ResolveContext<'static>,
) -> AstResult<()> {
    let mut ast = AstGenerator::new(context);

    for fn_item in defined_crate.fns.iter() {
        let context = ast.context();

        let ty_id = context[fn_item.item_id]
            .try_as_const()
            .map(|const_id| context[const_id].ty_id)
            .ok_or(ResolveError::ItemNotConst {
                item_id: fn_item.item_id,
            })?;

        ast.gen_fn_expr(&fn_item.item, fn_item.module_id, ty_id)?;
    }

    Ok(())
}
