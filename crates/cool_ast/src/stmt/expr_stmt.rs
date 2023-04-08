use crate::expr::ExprAst;
use crate::{AstGenerator, AstResult, ResolveAst, TyMismatch};
use cool_parser::ExprStmt;
use cool_resolve::{tys, ScopeId, TyId};

#[derive(Clone, Debug)]
pub struct ExprStmtAst {
    pub expr: ExprAst,
}

impl ResolveAst for ExprStmtAst {
    fn resolve(&self, ast: &mut AstGenerator, expected_ty: TyId) -> AstResult<TyId> {
        self.expr.resolve(ast, tys::INFERRED)?;

        if expected_ty != tys::UNIT {
            Err(TyMismatch {
                expected_ty,
                found_ty: tys::UNIT,
            })?
        }

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
