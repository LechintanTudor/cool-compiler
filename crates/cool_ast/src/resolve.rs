use crate::{AstGenerator, AstResult};
use cool_resolve::TyId;

pub trait ResolveAst {
    fn resolve(&self, ast: &mut AstGenerator, expected_ty: TyId) -> AstResult<TyId>;
}
