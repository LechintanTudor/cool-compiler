use crate::expr::{ExprAst, GenericExprAst};
use crate::{AstGenerator, AstResult, ResolveAst};
use cool_parser::ParenExpr;
use cool_resolve::{ExprId, ScopeId, TyId};

#[derive(Clone, Debug)]
pub struct ParenExprAst {
    pub id: ExprId,
    pub inner: Box<ExprAst>,
}

impl GenericExprAst for ParenExprAst {
    #[inline]
    fn id(&self) -> ExprId {
        self.id
    }
}

impl ResolveAst for ParenExprAst {
    fn resolve(&self, ast: &mut AstGenerator, expected_ty: TyId) -> AstResult<TyId> {
        let inner_ty = self.inner.resolve(ast, expected_ty)?;
        ast.resolve.set_expr_ty(self.id, inner_ty);
        Ok(inner_ty)
    }
}

impl AstGenerator<'_> {
    pub fn gen_paren_expr(&mut self, scope_id: ScopeId, expr: &ParenExpr) -> ParenExprAst {
        ParenExprAst {
            id: self.resolve.add_expr(),
            inner: Box::new(self.gen_expr(scope_id, &expr.inner)),
        }
    }
}
