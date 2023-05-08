use crate::{AstGenerator, AstResult, ExprAst, TyMismatch};
use cool_parser::IdentExpr;
use cool_resolve::{tys, BindingId, ExprId, FrameId, ItemKind, ModuleId, TyId};

#[derive(Clone, Debug)]
pub struct BindingExprAst {
    pub expr_id: ExprId,
    pub binding_id: BindingId,
}

#[derive(Clone, Debug)]
pub struct ModuleExprAst {
    pub expr_id: ExprId,
    pub module_id: ModuleId,
}

impl AstGenerator<'_> {
    pub fn gen_ident_expr(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        ident_expr: &IdentExpr,
    ) -> AstResult<ExprAst> {
        let item = self
            .resolve
            .resolve_local(frame_id, ident_expr.ident.symbol)?;

        let expr: ExprAst = match item {
            ItemKind::Binding(binding_id) => {
                let ty_id = self.resolve[binding_id]
                    .ty_id
                    .resolve_non_inferred(expected_ty_id)
                    .ok_or(TyMismatch {
                        found: self.resolve[binding_id].ty_id,
                        expected: expected_ty_id,
                    })?;

                let is_lvalue = self.resolve[binding_id].is_mutable();
                let expr_id = self.resolve.add_expr(ty_id, is_lvalue);

                BindingExprAst {
                    expr_id,
                    binding_id,
                }
                .into()
            }
            ItemKind::Module(module_id) => {
                let ty_id = tys::MODULE
                    .resolve_non_inferred(expected_ty_id)
                    .ok_or(TyMismatch {
                        found: tys::MODULE,
                        expected: expected_ty_id,
                    })?;

                let expr_id = self.resolve.add_expr(ty_id, true);
                ModuleExprAst { expr_id, module_id }.into()
            }
            _ => panic!("types are not allowed in expressions"),
        };

        Ok(expr)
    }
}
