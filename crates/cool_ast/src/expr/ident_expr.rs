use crate::{AstError, AstGenerator, AstResult, ExprAst, UnitExprAst};
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

        match item {
            ItemKind::Binding(binding_id) => {
                let found_ty_id = self.resolve[binding_id].ty_id;
                let is_mutable = self.resolve[binding_id].is_mutable();

                self.resolve_expr(
                    ident_expr.span(),
                    found_ty_id,
                    expected_ty_id,
                    |resolve, span, ty_id| {
                        BindingExprAst {
                            span,
                            expr_id: resolve.add_expr(ResolveExpr::lvalue(ty_id, is_mutable)),
                            binding_id,
                        }
                    },
                )
            }
            ItemKind::Ty(item_ty_id) => {
                if item_ty_id.is_unit() || item_ty_id.is_empty_struct() {
                    self.resolve_expr(
                        ident_expr.span(),
                        item_ty_id,
                        expected_ty_id,
                        |resolve, span, ty_id| {
                            UnitExprAst {
                                span,
                                expr_id: resolve.add_expr(ResolveExpr::rvalue(ty_id)),
                            }
                        },
                    )
                } else {
                    self.resolve_expr(
                        ident_expr.span(),
                        self.tys().ty,
                        expected_ty_id,
                        |resolve, span, ty_id| {
                            TyExprAst {
                                span,
                                expr_id: resolve.add_expr(ResolveExpr::lvalue(ty_id, false)),
                                item_ty_id,
                            }
                        },
                    )
                }
            }
            ItemKind::Module(module_id) => {
                self.resolve_expr(
                    ident_expr.span(),
                    self.tys().module,
                    expected_ty_id,
                    |resolve, span, ty_id| {
                        ModuleExprAst {
                            span,
                            expr_id: resolve.add_expr(ResolveExpr::lvalue(ty_id, false)),
                            module_id,
                        }
                    },
                )
            }
        }
    }
}
