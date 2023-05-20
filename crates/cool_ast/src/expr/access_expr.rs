use crate::{AstGenerator, AstResult, BindingExprAst, ExprAst, ModuleExprAst, TyExprAst};
use cool_parser::{AccessExpr, Ident};
use cool_resolve::{tys, ExprId, FrameId, ItemKind, ResolveExpr, TyId, ValueTy};

#[derive(Clone, Debug)]
pub struct StructAccessExprAst {
    pub expr_id: ExprId,
    pub base: Box<ExprAst>,
    pub ident: Ident,
}

impl AstGenerator<'_> {
    pub fn gen_access_expr(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        access_expr: &AccessExpr,
    ) -> AstResult<ExprAst> {
        let expr: ExprAst = match self.gen_expr(frame_id, tys::INFER, &access_expr.base)? {
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
                    ItemKind::Ty(ty_id) => {
                        self.resolve.resolve_direct_ty_id(tys::TY, expected_ty_id)?;

                        let expr_id = self.resolve.add_expr(ResolveExpr::ty());
                        TyExprAst {
                            expr_id,
                            item_ty_id: ty_id,
                        }
                        .into()
                    }
                    ItemKind::Module(module_id) => {
                        self.resolve
                            .resolve_direct_ty_id(tys::MODULE, expected_ty_id)?;

                        let expr_id = self.resolve.add_expr(ResolveExpr::module());
                        ModuleExprAst { expr_id, module_id }.into()
                    }
                }
            }
            base => {
                let base_expr = self.resolve[base.expr_id()];

                match &self.resolve[base_expr.ty_id].ty {
                    ValueTy::Struct(struct_ty) => {
                        let field_ty_id = struct_ty
                            .fields
                            .iter()
                            .find(|(field, _)| *field == access_expr.ident.symbol)
                            .map(|(_, ty_id)| *ty_id)
                            .expect("no field found");

                        let ty_id = self
                            .resolve
                            .resolve_direct_ty_id(field_ty_id, expected_ty_id)?;

                        let expr_id = self.resolve.add_expr(ResolveExpr { ty_id, ..base_expr });

                        StructAccessExprAst {
                            expr_id,
                            base: Box::new(base),
                            ident: access_expr.ident,
                        }
                        .into()
                    }
                    _ => todo!(),
                }
            }
        };

        Ok(expr)
    }
}
