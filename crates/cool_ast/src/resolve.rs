use crate::{AstGenerator, AstResult};
use cool_resolve::{tys, TyId};

pub trait ResolveAst {
    fn resolve_tys(&self, ast: &mut AstGenerator, expected_ty: TyId) -> AstResult<TyId> {
        let _ = (ast, expected_ty);
        Ok(tys::INFERRED)
    }

    fn resolve_consts(&self, ast: &mut AstGenerator, expected_ty: TyId) -> AstResult<TyId> {
        let _ = (ast, expected_ty);
        Ok(tys::INFERRED)
    }

    fn resolve_exprs(&self, ast: &mut AstGenerator, expected_ty: TyId) -> AstResult<TyId> {
        let _ = (ast, expected_ty);
        Ok(tys::INFERRED)
    }
}
