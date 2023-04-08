use crate::{AstGenerator, AstResult};
use cool_resolve::TyId;

pub trait ResolveAst {
    fn resolve_tys(&self, ast: &mut AstGenerator) -> AstResult {
        let _ = ast;
        Ok(())
    }

    fn resolve_fns(&self, ast: &mut AstGenerator) -> AstResult {
        let _ = ast;
        Ok(())
    }

    fn resolve_exprs(&self, ast: &mut AstGenerator, expected_ty: TyId) -> AstResult<TyId>;
}
