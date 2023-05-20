use crate::{AstGenerator, AstResult, ExprAst};
use cool_parser::IdentExpr;
use cool_resolve::{tys, BindingId, ExprId, FrameId, ItemKind, ModuleId, ResolveExpr, TyId};

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

#[derive(Clone, Debug)]
pub struct TyExprAst {
    pub expr_id: ExprId,
    pub item_ty_id: TyId,
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
        };

        Ok(expr)
    }
}
