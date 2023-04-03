use crate::expr::{ExprAst, GenericExprAst};
use crate::{AstGenerator, Unify};
use cool_parser::{DeclStmt, Pattern};
use cool_resolve::expr_ty::{Constraint, ExprTyUnifier};
use cool_resolve::resolve::{BindingId, FrameId, ScopeId};
use cool_resolve::ty::{TyId, TyTable};

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

impl AstGenerator<'_> {
    pub fn gen_decl_stmt(&mut self, scope_id: ScopeId, decl: &DeclStmt) -> DeclStmtAst {
        let frame_id = self.resolve.insert_frame(scope_id);

        let explicit_ty_id = decl
            .ty
            .as_ref()
            .map(|ty| self.resolve_ty(scope_id, ty).unwrap());

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
