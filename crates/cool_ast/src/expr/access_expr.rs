use crate::{
    AstGenerator, AstResult, BindingExprAst, DerefExprAst, ExprAst, ModuleExprAst, TyExprAst,
};
use cool_lexer::sym;
use cool_parser::{AccessExpr, Ident};
use cool_resolve::{tys, ExprId, FrameId, ItemKind, ResolveExpr, TyId, ValueTy};
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
                            span: access_expr.span(),
                            expr_id,
                            binding_id,
                        }
                        .into()
                    }
                    ItemKind::Ty(ty_id) => {
                        self.resolve.resolve_direct_ty_id(tys::TY, expected_ty_id)?;
                        let expr_id = self.resolve.add_expr(ResolveExpr::ty());

                        TyExprAst {
                            span: access_expr.span(),
                            expr_id,
                            item_ty_id: ty_id,
                        }
                        .into()
                    }
                    ItemKind::Module(module_id) => {
                        self.resolve
                            .resolve_direct_ty_id(tys::MODULE, expected_ty_id)?;
                        let expr_id = self.resolve.add_expr(ResolveExpr::module());

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
                let base_ty_id = self.resolve[base.expr_id()].ty_id;

                match self.resolve[base_ty_id].ty {
                    ValueTy::Ptr(_) => {
                        let new_base = self.gen_implicit_deref_expr(Box::new(base))?;

                        self.gen_aggregate_access_expr(
                            expected_ty_id,
                            Box::new(new_base.into()),
                            access_expr.ident,
                        )?
                    }
                    _ => {
                        self.gen_aggregate_access_expr(
                            expected_ty_id,
                            Box::new(base),
                            access_expr.ident,
                        )?
                    }
                }
            }
        };

        Ok(expr)
    }

    fn gen_implicit_deref_expr(&mut self, base: Box<ExprAst>) -> AstResult<DerefExprAst> {
        let base_expr = self.resolve[base.expr_id()];
        let base_ptr_ty = self.resolve[base_expr.ty_id].ty.as_ptr().unwrap();

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
        ident: Ident,
    ) -> AstResult<ExprAst> {
        let base_expr = self.resolve[base.expr_id()];

        let expr = match &self.resolve[base_expr.ty_id].ty {
            ValueTy::Aggregate(aggregate_ty) => {
                let field_ty_id = aggregate_ty
                    .get_field_ty_id(ident.symbol)
                    .expect("no field found");

                let ty_id = self
                    .resolve
                    .resolve_direct_ty_id(field_ty_id, expected_ty_id)?;

                let expr_id = self.resolve.add_expr(ResolveExpr { ty_id, ..base_expr });

                AccessExprAst {
                    expr_id,
                    base,
                    ident,
                }
                .into()
            }
            ValueTy::Array(_) => {
                if ident.symbol != sym::LEN {
                    panic!("unknown array access");
                }

                let ty_id = self
                    .resolve
                    .resolve_direct_ty_id(tys::USIZE, expected_ty_id)?;

                ArrayLenExprAst {
                    expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
                    base,
                    ident,
                }
                .into()
            }
            _ => todo!(),
        };

        Ok(expr)
    }
}
