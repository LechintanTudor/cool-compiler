use crate::{AstGenerator, SemanticResult};
use cool_resolve::ty::TyId;

pub trait ResolveAst {
    fn resolve(&self, ast: &mut AstGenerator, expected_ty: Option<TyId>) -> SemanticResult<TyId>;
}
