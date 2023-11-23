use crate::{resolve_fn, AstGenerator, ExprAst, SpannedAstResult, WithSpan};
use cool_derive::Section;
use cool_parser::FnExpr;
use cool_resolve::{Binding, ExprId, ModuleId, TyId};
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct FnExprAst {
    pub span: Span,
    pub expr_id: ExprId,
    pub body: Box<ExprAst>,
}

impl AstGenerator<'_> {
    pub fn gen_fn_expr(
        &mut self,
        expr: &FnExpr,
        module_id: ModuleId,
        expected_ty_id: TyId,
    ) -> SpannedAstResult<ExprAst> {
        let fn_ty_id = self.context[expected_ty_id]
            .try_as_fn()
            .map(|_| expected_ty_id);

        let ty_id = resolve_fn(self.context, module_id, fn_ty_id, &expr.prototype)
            .with_span(expr.prototype.span)?;

        let fn_ty = self.context[ty_id].try_as_fn().unwrap().clone();

        let params_frame_id = self.context.add_frame(module_id);
        for (param, &param_ty) in expr.prototype.params.iter().zip(fn_ty.param_tys.iter()) {
            self.context
                .add_binding(
                    params_frame_id,
                    Binding {
                        symbol: param.pattern.ident.symbol,
                        is_mutable: param.pattern.is_mutable,
                        ty_id: param_ty,
                    },
                )
                .with_span(param.span())?;
        }

        let body = self.gen_block_expr(&expr.body, params_frame_id, fn_ty.return_ty)?;

        self.resolve_expr(
            expr.span(),
            ty_id,
            expected_ty_id,
            |context, span, ty_id| {
                FnExprAst {
                    span,
                    expr_id: context.add_rvalue_expr(ty_id),
                    body: Box::new(body),
                }
            },
        )
    }
}
