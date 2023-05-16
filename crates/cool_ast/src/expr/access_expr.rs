use crate::{AstGenerator, AstResult, BindingExprAst, ExprAst, ModuleExprAst};
use cool_parser::AccessExpr;
use cool_resolve::{tys, FrameId, ItemKind, ResolveExpr, TyId};

impl AstGenerator<'_> {
    pub fn gen_access_expr(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        access_expr: &AccessExpr,
    ) -> AstResult<ExprAst> {
        let base = self.gen_expr(frame_id, tys::INFER, &access_expr.base)?;

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
                        let ty_id = self
                            .resolve
                            .resolve_direct_ty_id(self.resolve[binding_id].ty_id, expected_ty_id)?;

                        let is_mutable = self.resolve[binding_id].is_mutable();

                        let expr_id = self
                            .resolve
                            .add_expr(ResolveExpr::lvalue(ty_id, is_mutable));

                        BindingExprAst {
                            expr_id,
                            binding_id,
                        }
                        .into()
                    }
                    ItemKind::Module(module_id) => {
                        self.resolve
                            .resolve_direct_ty_id(tys::MODULE, expected_ty_id)?;

                        let expr_id = self.resolve.add_expr(ResolveExpr::module());
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
