use crate::{
    AstError, AstGenerator, AstResult, AstResultExt, BindingExprAst, DerefExprAst, ExprAst,
    ModuleExprAst, TyExprAst,
};
use cool_lexer::sym;
use cool_parser::{AccessExpr, Ident};
use cool_resolve::{ExprId, FrameId, ItemKind, ResolveExpr, TyId, TyShape, ValueTy};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct AccessExprAst {
    pub expr_id: ExprId,
    pub base: Box<ExprAst>,
    pub ident: Ident,
}

impl Section for AccessExprAst {
    #[inline]
    fn span(&self) -> Span {
        self.base.span().to(self.ident.span())
    }
}

#[derive(Clone, Debug)]
pub struct ArrayLenExprAst {
    pub expr_id: ExprId,
    pub base: Box<ExprAst>,
    pub ident: Ident,
}

impl Section for ArrayLenExprAst {
    #[inline]
    fn span(&self) -> Span {
        self.base.span().to(self.ident.span())
    }
}

impl AstGenerator<'_> {
    pub fn gen_access_expr(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        access_expr: &AccessExpr,
    ) -> AstResult<ExprAst> {
        let expr: ExprAst = match self.gen_expr(frame_id, self.tys().infer, &access_expr.base)? {
            ExprAst::Module(module_expr) => {
                let parent_module_id = self.resolve.resolve_parent_module(frame_id.into());

                let item = self
                    .resolve
                    .resolve_local_access(
                        parent_module_id,
                        module_expr.module_id,
                        access_expr.ident.symbol,
                    )
                    .map_err(|error| AstError::new(access_expr.span(), error))?;

                match item {
                    ItemKind::Binding(binding_id) => {
                        let ty_id = self.resolve_direct_ty_id(
                            access_expr.span(),
                            self.resolve[binding_id].ty_id,
                            expected_ty_id,
                        )?;

                        let is_mutable = self.resolve[binding_id].is_mutable();

                        let expr_id = self
                            .resolve
                            .add_expr(ResolveExpr::lvalue(ty_id, is_mutable));

                        BindingExprAst {
                            span: access_expr.span(),
                            expr_id,
                            binding_id,
                        }
                        .into()
                    }
                    ItemKind::Ty(ty_id) => {
                        self.resolve_direct_ty_id(
                            access_expr.span(),
                            self.tys().ty,
                            expected_ty_id,
                        )?;

                        let expr_id = self
                            .resolve
                            .add_expr(ResolveExpr::lvalue(self.tys().ty, false));

                        TyExprAst {
                            span: access_expr.span(),
                            expr_id,
                            item_ty_id: ty_id,
                        }
                        .into()
                    }
                    ItemKind::Module(module_id) => {
                        self.resolve_direct_ty_id(
                            access_expr.span(),
                            self.tys().module,
                            expected_ty_id,
                        )?;

                        let expr_id = self
                            .resolve
                            .add_expr(ResolveExpr::lvalue(self.tys().module, false));

                        ModuleExprAst {
                            span: access_expr.span(),
                            expr_id,
                            module_id,
                        }
                        .into()
                    }
                }
            }
            base => {
                match &*base.expr_id().ty_id {
                    TyShape::Value(ValueTy::Ptr(_)) => {
                        let new_base = self.gen_implicit_deref_expr(Box::new(base))?;

                        self.gen_aggregate_access_expr(
                            expected_ty_id,
                            Box::new(new_base.into()),
                            access_expr,
                        )?
                    }
                    _ => {
                        self.gen_aggregate_access_expr(expected_ty_id, Box::new(base), access_expr)?
                    }
                }
            }
        };

        Ok(expr)
    }

    fn gen_implicit_deref_expr(&mut self, base: Box<ExprAst>) -> AstResult<DerefExprAst> {
        let base_ty_id = base.expr_id().ty_id;
        let base_ptr_ty = base_ty_id.as_ptr().unwrap();

        let expr_id = self.resolve.add_expr(ResolveExpr::lvalue(
            base_ptr_ty.pointee,
            base_ptr_ty.is_mutable,
        ));

        Ok(DerefExprAst {
            span: Span::new(base.span().start, 0),
            expr_id,
            expr: base,
        })
    }

    fn gen_aggregate_access_expr(
        &mut self,
        expected_ty_id: TyId,
        base: Box<ExprAst>,
        access_expr: &AccessExpr,
    ) -> AstResult<ExprAst> {
        let base_expr_id = base.expr_id();
        let ident = access_expr.ident;

        let expr = if base_expr_id.ty_id.is_array() {
            if ident.symbol != sym::LEN {
                return AstResult::field_not_found(
                    access_expr.span(),
                    base_expr_id.ty_id,
                    ident.symbol,
                );
            }

            let ty_id =
                self.resolve_direct_ty_id(access_expr.span(), self.tys().usize, expected_ty_id)?;

            ExprAst::from(ArrayLenExprAst {
                expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
                base,
                ident,
            })
        } else {
            let field = self
                .resolve
                .get_ty_def(base_expr_id.ty_id)
                .unwrap()
                .get_aggregate_field(ident.symbol)
                .ok_or_else(|| {
                    AstError::field_not_found(access_expr.span(), base_expr_id.ty_id, ident.symbol)
                })?;

            let ty_id =
                self.resolve_direct_ty_id(access_expr.span(), field.ty_id, expected_ty_id)?;

            let expr_id = self.resolve.add_expr(ResolveExpr {
                ty_id,
                ..*base_expr_id
            });

            ExprAst::from(AccessExprAst {
                expr_id,
                base,
                ident,
            })
        };

        Ok(expr)
    }
}
