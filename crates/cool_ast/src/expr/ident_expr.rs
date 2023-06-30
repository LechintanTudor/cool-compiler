use crate::{AstError, AstGenerator, AstResult, ExprAst};
use cool_parser::IdentExpr;
use cool_resolve::{BindingId, ExprId, FrameId, ItemKind, ModuleId, ResolveExpr, TyId};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct BindingExprAst {
    pub span: Span,
    pub expr_id: ExprId,
    pub binding_id: BindingId,
}

impl Section for BindingExprAst {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone, Debug)]
pub struct ModuleExprAst {
    pub span: Span,
    pub expr_id: ExprId,
    pub module_id: ModuleId,
}

impl Section for ModuleExprAst {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone, Debug)]
pub struct TyExprAst {
    pub span: Span,
    pub expr_id: ExprId,
    pub item_ty_id: TyId,
}

impl Section for TyExprAst {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
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
            .resolve_local(frame_id, ident_expr.ident.symbol)
            .map_err(|error| AstError::new(ident_expr.span(), error))?;

        let expr: ExprAst = match item {
            ItemKind::Binding(binding_id) => {
                let ty_id = self.resolve_ty_id(
                    ident_expr.span(),
                    self.resolve[binding_id].ty_id,
                    expected_ty_id,
                )?;

                let is_mutable = self.resolve[binding_id].is_mutable();

                let expr_id = self
                    .resolve
                    .add_expr(ResolveExpr::lvalue(ty_id, is_mutable));

                BindingExprAst {
                    span: ident_expr.span(),
                    expr_id,
                    binding_id,
                }
                .into()
            }
            ItemKind::Ty(ty_id) => {
                self.resolve_ty_id(ident_expr.span(), self.tys().ty, expected_ty_id)?;

                let expr_id = self
                    .resolve
                    .add_expr(ResolveExpr::lvalue(self.tys().ty, false));

                TyExprAst {
                    span: ident_expr.span(),
                    expr_id,
                    item_ty_id: ty_id,
                }
                .into()
            }
            ItemKind::Module(module_id) => {
                self.resolve_ty_id(ident_expr.span(), self.tys().module, expected_ty_id)?;

                let expr_id = self
                    .resolve
                    .add_expr(ResolveExpr::lvalue(self.tys().module, false));

                ModuleExprAst {
                    span: ident_expr.span(),
                    expr_id,
                    module_id,
                }
                .into()
            }
        };

        Ok(expr)
    }
}
