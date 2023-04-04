use crate::expr::{ExprAst, GenericExprAst};
use crate::{AstGenerator, ResolveAst, SemanticResult, TyMismatch, Unify};
use cool_parser::{DeclStmt, Pattern};
use cool_resolve::expr_ty::{Constraint, ExprTyUnifier};
use cool_resolve::resolve::{BindingId, FrameId, ScopeId};
use cool_resolve::ty::{tys, TyId, TyTable};

#[derive(Clone, Debug)]
pub struct DeclStmtAst {
    pub frame_id: FrameId,
    pub binding_id: BindingId,
    pub pattern: Pattern,
    pub explicit_ty_id: Option<TyId>,
    pub expr: ExprAst,
}

impl Unify for DeclStmtAst {
    fn unify(&self, unifier: &mut ExprTyUnifier, tys: &mut TyTable) {
        self.expr.unify(unifier, tys);

        match self.explicit_ty_id {
            Some(explicit_ty_id) => {
                unifier.add_constraint(
                    Constraint::Binding(self.binding_id),
                    Constraint::Ty(explicit_ty_id),
                );
            }
            None => {
                let ty_var_id = unifier.add_ty_var();

                unifier.add_constraint(
                    Constraint::Binding(self.binding_id),
                    Constraint::TyVar(ty_var_id),
                )
            }
        }

        unifier.add_constraint(
            Constraint::Binding(self.binding_id),
            Constraint::Expr(self.expr.id()),
        );
    }
}

impl ResolveAst for DeclStmtAst {
    fn resolve(&self, ast: &mut AstGenerator, expected_ty: Option<TyId>) -> SemanticResult<TyId> {
        let expr_ty = self.expr.resolve(ast, self.explicit_ty_id)?;
        ast.expr_tys.set_binding_ty(self.binding_id, expr_ty);

        let resolved_ty = tys::UNIT
            .resolve_non_inferred(expected_ty)
            .ok_or(TyMismatch {
                found_ty: tys::UNIT,
                expected_ty,
            })?;

        Ok(resolved_ty)
    }
}

impl AstGenerator<'_> {
    pub fn gen_decl_stmt(&mut self, scope_id: ScopeId, decl: &DeclStmt) -> DeclStmtAst {
        let frame_id = self.resolve.insert_frame(scope_id);

        let explicit_ty_id = decl
            .ty
            .as_ref()
            .map(|ty| self.resolve_parsed_ty(scope_id, ty).unwrap());

        let binding_id = self
            .resolve
            .insert_local_binding(frame_id, decl.pattern.is_mutable, decl.pattern.ident.symbol)
            .unwrap();

        let expr = self.gen_expr(scope_id, &decl.expr);

        DeclStmtAst {
            frame_id,
            binding_id,
            pattern: decl.pattern.clone(),
            explicit_ty_id,
            expr,
        }
    }
}
