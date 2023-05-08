use crate::{AstGenerator, AstResult, BindingExprAst, ExprAst, ModuleExprAst, TyMismatch};
use cool_parser::AccessExpr;
use cool_resolve::{tys, FrameId, ItemKind, TyId};

impl AstGenerator<'_> {
    pub fn gen_access_expr(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        access_expr: &AccessExpr,
    ) -> AstResult<ExprAst> {
        let base = self.gen_expr(frame_id, tys::INFERRED, &access_expr.base)?;

        let expr: ExprAst = match base {
            ExprAst::Module(module_expr) => {
                let parent_module_id = self.resolve.resolve_parent_module(frame_id.into());

                let item = self.resolve.resolve_local_access(
                    parent_module_id,
                    module_expr.module_id,
                    access_expr.ident.symbol,
                )?;

                match item {
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
                        tys::MODULE
                            .resolve_non_inferred(expected_ty_id)
                            .ok_or(TyMismatch {
                                found: tys::MODULE,
                                expected: expected_ty_id,
                            })?;

                        let expr_id = self.resolve.add_expr(tys::MODULE, true);
                        ModuleExprAst { expr_id, module_id }.into()
                    }
                    _ => panic!("types are not allowed here"),
                }
            }
            _ => todo!(),
        };

        Ok(expr)
    }
}
