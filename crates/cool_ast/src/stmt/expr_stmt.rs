use crate::expr::ExprAst;
use crate::{AstGenerator, AstResult, ResolveAst, TyMismatch};
use cool_parser::ExprStmt;
use cool_resolve::{tys, ScopeId, TyId};

#[derive(Clone, Debug)]
pub struct ExprStmtAst {
    pub expr: ExprAst,
}

impl ResolveAst for ExprStmtAst {
    fn resolve_exprs(&self, ast: &mut AstGenerator, expected_ty: TyId) -> AstResult<TyId> {
        self.expr.resolve_exprs(ast, tys::INFERRED)?;

        tys::UNIT
            .resolve_non_inferred(expected_ty)
            .ok_or(TyMismatch {
                expected_ty,
                found_ty: tys::UNIT,
            })?;

        Ok(tys::UNIT)
    }
}

impl AstGenerator<'_> {
    pub fn gen_expr_stmt(&mut self, scope_id: ScopeId, stmt: &ExprStmt) -> ExprStmtAst {
        ExprStmtAst {
            expr: self.gen_expr(scope_id, &stmt.expr),
        }
    }
}
